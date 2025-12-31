use crate::config::Config;
use crate::models::{Booking, Room, User};
use chrono::Local;

/// Strategy Pattern - Different validation strategies for bookings
pub trait ValidationStrategy {
    fn validate(
        &self,
        booking: &Booking,
        room: &Room,
        user: &User,
        existing_bookings: &[Booking],
    ) -> Result<(), String>;
}

/// Standard validation - Basic business rules
pub struct StandardValidation;

impl ValidationStrategy for StandardValidation {
    fn validate(
        &self,
        booking: &Booking,
        room: &Room,
        _user: &User,
        existing_bookings: &[Booking],
    ) -> Result<(), String> {
        let config = Config::get();

        // Check if booking is in the past
        if booking.start_time < Local::now() {
            return Err("Cannot book in the past".to_string());
        }

        // Check duration
        let duration = booking.duration_hours();
        if duration < config.min_booking_duration_hours as f64 {
            return Err(format!(
                "Booking must be at least {} hours",
                config.min_booking_duration_hours
            ));
        }
        if duration > config.max_booking_duration_hours as f64 {
            return Err(format!(
                "Booking cannot exceed {} hours",
                config.max_booking_duration_hours
            ));
        }

        // Check advance booking
        let days_advance = (booking.start_time - Local::now()).num_days();
        if days_advance > config.max_advance_booking_days as i64 {
            return Err(format!(
                "Cannot book more than {} days in advance",
                config.max_advance_booking_days
            ));
        }

        // Check capacity
        if booking.attendees > room.capacity {
            return Err(format!(
                "Room capacity ({}) exceeded by attendees ({})",
                room.capacity, booking.attendees
            ));
        }

        // Check for overlapping bookings
        for existing in existing_bookings {
            if booking.overlaps_with(existing) {
                return Err("Room is already booked for this time period".to_string());
            }
        }

        // Check business hours
        let hour = booking.start_time.hour();
        if hour < config.business_hours_start || hour >= config.business_hours_end {
            return Err(format!(
                "Bookings must be within business hours ({}-{})",
                config.business_hours_start, config.business_hours_end
            ));
        }

        Ok(())
    }
}

/// Priority validation - For high-priority users (managers, executives)
pub struct PriorityValidation;

impl ValidationStrategy for PriorityValidation {
    fn validate(
        &self,
        booking: &Booking,
        room: &Room,
        user: &User,
        existing_bookings: &[Booking],
    ) -> Result<(), String> {
        // Less strict validation for priority users
        if booking.start_time < Local::now() {
            return Err("Cannot book in the past".to_string());
        }

        // Allow capacity overflow for managers
        if booking.attendees > room.capacity + 2 && !user.can_manage_bookings() {
            return Err("Room capacity exceeded".to_string());
        }

        // Allow some booking conflicts for priority users (would need approval)
        let conflicts: Vec<_> = existing_bookings
            .iter()
            .filter(|b| booking.overlaps_with(b))
            .collect();

        if conflicts.len() > 1 {
            return Err("Too many conflicting bookings".to_string());
        }

        Ok(())
    }
}

/// Flexible validation - Minimal restrictions
pub struct FlexibleValidation;

impl ValidationStrategy for FlexibleValidation {
    fn validate(
        &self,
        booking: &Booking,
        _room: &Room,
        _user: &User,
        _existing_bookings: &[Booking],
    ) -> Result<(), String> {
        if booking.start_time < Local::now() {
            return Err("Cannot book in the past".to_string());
        }
        Ok(())
    }
}

pub struct BookingValidator {
    strategy: Box<dyn ValidationStrategy>,
}

impl BookingValidator {
    pub fn new(strategy: Box<dyn ValidationStrategy>) -> Self {
        Self { strategy }
    }

    pub fn set_strategy(&mut self, strategy: Box<dyn ValidationStrategy>) {
        self.strategy = strategy;
    }

    pub fn validate(
        &self,
        booking: &Booking,
        room: &Room,
        user: &User,
        existing_bookings: &[Booking],
    ) -> Result<(), String> {
        self.strategy.validate(booking, room, user, existing_bookings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{RoomType, UserRole};
    use chrono::Duration;
    use uuid::Uuid;

    #[test]
    fn test_standard_validation() {
        let validator = BookingValidator::new(Box::new(StandardValidation));
        let room = Room::new("Test".to_string(), RoomType::MeetingRoom, 10, 1);
        let user = User::new("Test".to_string(), "test@test.com".to_string(), "IT".to_string());
        
        let booking = Booking::new(
            room.id,
            user.id,
            Local::now() + Duration::hours(1),
            Local::now() + Duration::hours(2),
            "Test".to_string(),
            5,
        );

        assert!(validator.validate(&booking, &room, &user, &[]).is_ok());
    }

    #[test]
    fn test_validation_strategy_change() {
        let mut validator = BookingValidator::new(Box::new(StandardValidation));
        validator.set_strategy(Box::new(FlexibleValidation));
        // Strategy changed successfully
    }
}
