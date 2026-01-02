// =============================================================================
// STRATEGY PATTERN - Reservation Validation Strategies
// =============================================================================
// Problem Solved: Allows different validation rules for reservations to be
//                 swapped at runtime. Different business rules for different
//                 users (standard, VIP, admin) or different booking contexts.
// Location: Used by ReservationService when validating new reservations
// =============================================================================

use crate::models::reservation::{Reservation, ReservationStatus};
use crate::models::room::Room;
use crate::patterns::singleton::{log_info, log_warning, CONFIG};
use chrono::{DateTime, Datelike, Duration, Utc, Weekday, Timelike};
use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
// Validation Result Type
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub suggested_alternatives: Vec<String>,
}

impl ValidationResult {
    pub fn success() -> Self {
        ValidationResult {
            is_valid: true,
            errors: vec![],
            warnings: vec![],
            suggested_alternatives: vec![],
        }
    }

    pub fn failure(errors: Vec<String>) -> Self {
        ValidationResult {
            is_valid: false,
            errors,
            warnings: vec![],
            suggested_alternatives: vec![],
        }
    }

    pub fn with_warning(mut self, warning: String) -> Self {
        self.warnings.push(warning);
        self
    }

    pub fn with_suggestion(mut self, suggestion: String) -> Self {
        self.suggested_alternatives.push(suggestion);
        self
    }

    pub fn merge(mut self, other: ValidationResult) -> Self {
        if !other.is_valid {
            self.is_valid = false;
        }
        self.errors.extend(other.errors);
        self.warnings.extend(other.warnings);
        self.suggested_alternatives.extend(other.suggested_alternatives);
        self
    }
}

// -----------------------------------------------------------------------------
// Validation Strategy Trait
// -----------------------------------------------------------------------------
pub trait ValidationStrategy: Send + Sync {
    fn validate(
        &self,
        reservation: &Reservation,
        room: &Room,
        existing_reservations: &[Reservation],
    ) -> ValidationResult;

    fn strategy_name(&self) -> &str;
    fn strategy_description(&self) -> &str;
}

// -----------------------------------------------------------------------------
// Standard Validation Strategy - For regular users
// -----------------------------------------------------------------------------
pub struct StandardValidationStrategy;

impl ValidationStrategy for StandardValidationStrategy {
    fn validate(
        &self,
        reservation: &Reservation,
        room: &Room,
        existing_reservations: &[Reservation],
    ) -> ValidationResult {
        log_info(
            &format!(
                "Validating reservation {} with Standard strategy",
                reservation.id
            ),
            Some("StandardValidation"),
        );

        let config = CONFIG.get();
        let mut result = ValidationResult::success();

        // Check room availability
        if !room.is_available {
            return ValidationResult::failure(vec![
                "Room is not available for booking".to_string()
            ]);
        }

        // Check attendees vs capacity
        if reservation.attendees > room.capacity {
            return ValidationResult::failure(vec![format!(
                "Number of attendees ({}) exceeds room capacity ({})",
                reservation.attendees, room.capacity
            )])
            .with_suggestion(format!(
                "Consider a larger room or reduce attendees to {} or fewer",
                room.capacity
            ));
        }

        // Check time validity
        if reservation.start_time >= reservation.end_time {
            return ValidationResult::failure(vec![
                "End time must be after start time".to_string()
            ]);
        }

        // Check minimum duration
        let duration = reservation.end_time - reservation.start_time;
        let min_minutes = config.min_reservation_duration_minutes as i64;
        if duration < Duration::minutes(min_minutes) {
            return ValidationResult::failure(vec![format!(
                "Reservation must be at least {} minutes",
                min_minutes
            )]);
        }

        // Check maximum duration
        let max_hours = config.max_reservation_duration_hours as i64;
        if duration > Duration::hours(max_hours) {
            return ValidationResult::failure(vec![format!(
                "Reservation cannot exceed {} hours",
                max_hours
            )]);
        }

        // Check business hours
        let start_hour = reservation.start_time.hour();
        let end_hour = reservation.end_time.hour();
        if start_hour < config.business_hours_start as u32
            || end_hour > config.business_hours_end as u32
        {
            result = result.with_warning(format!(
                "Reservation is outside business hours ({:02}:00 - {:02}:00)",
                config.business_hours_start, config.business_hours_end
            ));
        }

        // Check weekend restrictions
        let start_weekday = reservation.start_time.weekday();
        if !config.allow_weekend_reservations
            && (start_weekday == Weekday::Sat || start_weekday == Weekday::Sun)
        {
            return ValidationResult::failure(vec![
                "Weekend reservations are not allowed".to_string()
            ]);
        }

        // Check advance booking limit
        let now = Utc::now();
        let days_ahead = (reservation.start_time - now).num_days();
        if days_ahead > config.max_reservation_days as i64 {
            return ValidationResult::failure(vec![format!(
                "Cannot book more than {} days in advance",
                config.max_reservation_days
            )]);
        }

        // Check for conflicts
        if let Some(conflict) = self.find_conflict(reservation, existing_reservations) {
            return ValidationResult::failure(vec![format!(
                "Time slot conflicts with existing reservation from {} to {}",
                conflict.start_time.format("%H:%M"),
                conflict.end_time.format("%H:%M")
            )]);
        }

        result
    }

    fn strategy_name(&self) -> &str {
        "Standard"
    }

    fn strategy_description(&self) -> &str {
        "Standard validation for regular users with all business rules enforced"
    }
}

impl StandardValidationStrategy {
    fn find_conflict<'a>(
        &self,
        reservation: &Reservation,
        existing: &'a [Reservation],
    ) -> Option<&'a Reservation> {
        existing.iter().find(|r| {
            r.room_id == reservation.room_id
                && r.status != ReservationStatus::Cancelled
                && r.id != reservation.id
                && reservation.start_time < r.end_time
                && reservation.end_time > r.start_time
        })
    }
}

// -----------------------------------------------------------------------------
// VIP Validation Strategy - For premium users with relaxed rules
// -----------------------------------------------------------------------------
pub struct VipValidationStrategy;

impl ValidationStrategy for VipValidationStrategy {
    fn validate(
        &self,
        reservation: &Reservation,
        room: &Room,
        existing_reservations: &[Reservation],
    ) -> ValidationResult {
        log_info(
            &format!(
                "Validating reservation {} with VIP strategy",
                reservation.id
            ),
            Some("VipValidation"),
        );

        let mut result = ValidationResult::success();

        // VIP users can book unavailable rooms (with warning)
        if !room.is_available {
            result = result.with_warning(
                "Room is marked as unavailable - VIP override applied".to_string(),
            );
        }

        // Check attendees with 20% buffer for VIP
        let max_capacity = (room.capacity as f64 * 1.2) as u32;
        if reservation.attendees > max_capacity {
            return ValidationResult::failure(vec![format!(
                "Number of attendees ({}) exceeds VIP capacity limit ({})",
                reservation.attendees, max_capacity
            )]);
        }

        // Check time validity
        if reservation.start_time >= reservation.end_time {
            return ValidationResult::failure(vec![
                "End time must be after start time".to_string()
            ]);
        }

        // VIP: Extended max duration (12 hours)
        let duration = reservation.end_time - reservation.start_time;
        if duration > Duration::hours(12) {
            return ValidationResult::failure(vec![
                "VIP reservations cannot exceed 12 hours".to_string()
            ]);
        }

        // VIP: Can book further in advance (180 days)
        let now = Utc::now();
        let days_ahead = (reservation.start_time - now).num_days();
        if days_ahead > 180 {
            return ValidationResult::failure(vec![
                "Cannot book more than 180 days in advance".to_string()
            ]);
        }

        // Still check for conflicts, but warn instead of block for some
        for existing in existing_reservations {
            if existing.room_id == reservation.room_id
                && existing.status != ReservationStatus::Cancelled
                && existing.id != reservation.id
                && reservation.start_time < existing.end_time
                && reservation.end_time > existing.start_time
            {
                // VIP can see conflicts but needs manual resolution
                result = result.with_warning(format!(
                    "Overlaps with reservation by {} ({} - {}). Contact admin to resolve.",
                    existing.user_name,
                    existing.start_time.format("%H:%M"),
                    existing.end_time.format("%H:%M")
                ));
            }
        }

        result
    }

    fn strategy_name(&self) -> &str {
        "VIP"
    }

    fn strategy_description(&self) -> &str {
        "VIP validation with extended limits: +20% capacity, 12h max duration, 180-day advance booking"
    }
}

// -----------------------------------------------------------------------------
// Admin Validation Strategy - Minimal restrictions for administrators
// -----------------------------------------------------------------------------
pub struct AdminValidationStrategy;

impl ValidationStrategy for AdminValidationStrategy {
    fn validate(
        &self,
        reservation: &Reservation,
        room: &Room,
        existing_reservations: &[Reservation],
    ) -> ValidationResult {
        log_info(
            &format!(
                "Validating reservation {} with Admin strategy",
                reservation.id
            ),
            Some("AdminValidation"),
        );

        let mut result = ValidationResult::success();

        // Only basic sanity checks for admin
        if reservation.start_time >= reservation.end_time {
            return ValidationResult::failure(vec![
                "End time must be after start time".to_string()
            ]);
        }

        // Warn about conflicts but don't block
        for existing in existing_reservations {
            if existing.room_id == reservation.room_id
                && existing.status != ReservationStatus::Cancelled
                && existing.id != reservation.id
                && reservation.start_time < existing.end_time
                && reservation.end_time > existing.start_time
            {
                result = result.with_warning(format!(
                    "Admin override: Overlapping reservation by {} will be affected",
                    existing.user_name
                ));
            }
        }

        // Warn about capacity overflow
        if reservation.attendees > room.capacity {
            result = result.with_warning(format!(
                "Attendees ({}) exceed capacity ({}) - admin override",
                reservation.attendees, room.capacity
            ));
        }

        result
    }

    fn strategy_name(&self) -> &str {
        "Admin"
    }

    fn strategy_description(&self) -> &str {
        "Admin validation with minimal restrictions - only basic time checks enforced"
    }
}

// -----------------------------------------------------------------------------
// Quiet Hours Strategy - For after-hours bookings with extra rules
// -----------------------------------------------------------------------------
pub struct QuietHoursValidationStrategy;

impl ValidationStrategy for QuietHoursValidationStrategy {
    fn validate(
        &self,
        reservation: &Reservation,
        room: &Room,
        existing_reservations: &[Reservation],
    ) -> ValidationResult {
        log_info(
            &format!(
                "Validating reservation {} with Quiet Hours strategy",
                reservation.id
            ),
            Some("QuietHoursValidation"),
        );

        // First apply standard validation
        let standard = StandardValidationStrategy;
        let mut result = standard.validate(reservation, room, existing_reservations);

        // Additional quiet hours rules
        let start_hour = reservation.start_time.hour();
        let end_hour = reservation.end_time.hour();

        // Quiet hours: 22:00 - 06:00
        let is_quiet_hours = start_hour >= 22 || start_hour < 6 || end_hour >= 22 || end_hour < 6;

        if is_quiet_hours {
            // Only certain room types allowed during quiet hours
            let quiet_allowed = matches!(
                room.room_type,
                crate::models::room::RoomType::PrivateOffice
                    | crate::models::room::RoomType::Meeting
            );

            if !quiet_allowed {
                result = result.merge(ValidationResult::failure(vec![format!(
                    "{} rooms are not available during quiet hours (22:00 - 06:00)",
                    room.room_type.as_str()
                )]));
            }

            // Max 2 hour duration during quiet hours
            let duration = reservation.end_time - reservation.start_time;
            if duration > Duration::hours(2) {
                result = result.merge(ValidationResult::failure(vec![
                    "Quiet hours reservations limited to 2 hours maximum".to_string(),
                ]));
            }

            result = result.with_warning(
                "This reservation includes quiet hours - please be mindful of noise levels"
                    .to_string(),
            );
        }

        result
    }

    fn strategy_name(&self) -> &str {
        "QuietHours"
    }

    fn strategy_description(&self) -> &str {
        "Quiet hours validation (22:00-06:00): Limited room types, 2h max duration"
    }
}

// -----------------------------------------------------------------------------
// Strategy Context - Manages and applies validation strategies
// -----------------------------------------------------------------------------
pub struct ValidationContext {
    strategy: Box<dyn ValidationStrategy>,
}

impl ValidationContext {
    pub fn new(strategy: Box<dyn ValidationStrategy>) -> Self {
        log_info(
            &format!("Created validation context with {} strategy", strategy.strategy_name()),
            Some("ValidationContext"),
        );
        ValidationContext { strategy }
    }

    pub fn validate(
        &self,
        reservation: &Reservation,
        room: &Room,
        existing_reservations: &[Reservation],
    ) -> ValidationResult {
        self.strategy.validate(reservation, room, existing_reservations)
    }

    pub fn set_strategy(&mut self, strategy: Box<dyn ValidationStrategy>) {
        log_info(
            &format!("Changed validation strategy to {}", strategy.strategy_name()),
            Some("ValidationContext"),
        );
        self.strategy = strategy;
    }

    pub fn get_strategy_info(&self) -> StrategyInfo {
        StrategyInfo {
            name: self.strategy.strategy_name().to_string(),
            description: self.strategy.strategy_description().to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyInfo {
    pub name: String,
    pub description: String,
}

// Factory function to get strategy by user role
pub fn get_strategy_for_role(role: &str) -> Box<dyn ValidationStrategy> {
    log_info(
        &format!("Getting validation strategy for role: {}", role),
        Some("Strategy"),
    );
    
    match role.to_lowercase().as_str() {
        "admin" | "administrator" => Box::new(AdminValidationStrategy),
        "vip" | "premium" | "executive" => Box::new(VipValidationStrategy),
        "quiet" | "afterhours" => Box::new(QuietHoursValidationStrategy),
        _ => Box::new(StandardValidationStrategy),
    }
}

/// Get all available strategies
pub fn get_available_strategies() -> Vec<StrategyInfo> {
    vec![
        StrategyInfo {
            name: "Standard".to_string(),
            description: StandardValidationStrategy.strategy_description().to_string(),
        },
        StrategyInfo {
            name: "VIP".to_string(),
            description: VipValidationStrategy.strategy_description().to_string(),
        },
        StrategyInfo {
            name: "Admin".to_string(),
            description: AdminValidationStrategy.strategy_description().to_string(),
        },
        StrategyInfo {
            name: "QuietHours".to_string(),
            description: QuietHoursValidationStrategy.strategy_description().to_string(),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::room::{RoomEquipment, RoomType};
    use uuid::Uuid;

    fn create_test_room() -> Room {
        Room::new(
            Uuid::new_v4().to_string(),
            "Test Room".to_string(),
            RoomType::Meeting,
            10,
            1,
            vec![RoomEquipment::WiFi],
            25.0,
        )
    }

    fn create_test_reservation(
        room_id: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        attendees: u32,
    ) -> Reservation {
        Reservation::new(
            room_id.to_string(),
            "Test User".to_string(),
            "test@example.com".to_string(),
            start,
            end,
            attendees,
            Some("Test reservation".to_string()),
        )
    }

    #[test]
    fn test_standard_validation_success() {
        let room = create_test_room();
        let now = Utc::now();
        let reservation = create_test_reservation(
            &room.id,
            now + Duration::hours(1),
            now + Duration::hours(2),
            5,
        );

        let strategy = StandardValidationStrategy;
        let result = strategy.validate(&reservation, &room, &[]);

        assert!(result.is_valid);
    }

    #[test]
    fn test_standard_validation_capacity_exceeded() {
        let room = create_test_room();
        let now = Utc::now();
        let reservation = create_test_reservation(
            &room.id,
            now + Duration::hours(1),
            now + Duration::hours(2),
            15, // Exceeds capacity of 10
        );

        let strategy = StandardValidationStrategy;
        let result = strategy.validate(&reservation, &room, &[]);

        assert!(!result.is_valid);
        assert!(result.errors[0].contains("exceeds room capacity"));
    }

    #[test]
    fn test_vip_extended_capacity() {
        let room = create_test_room();
        let now = Utc::now();
        let reservation = create_test_reservation(
            &room.id,
            now + Duration::hours(1),
            now + Duration::hours(2),
            12, // 120% of capacity, VIP allows this
        );

        let strategy = VipValidationStrategy;
        let result = strategy.validate(&reservation, &room, &[]);

        assert!(result.is_valid);
    }

    #[test]
    fn test_strategy_switching() {
        let room = create_test_room();
        let now = Utc::now();
        let reservation = create_test_reservation(
            &room.id,
            now + Duration::hours(1),
            now + Duration::hours(2),
            12,
        );

        let mut context = ValidationContext::new(Box::new(StandardValidationStrategy));
        let result1 = context.validate(&reservation, &room, &[]);
        assert!(!result1.is_valid); // Standard fails

        context.set_strategy(Box::new(VipValidationStrategy));
        let result2 = context.validate(&reservation, &room, &[]);
        assert!(result2.is_valid); // VIP succeeds
    }
}
