// =============================================================================
// RESERVATION MODEL
// =============================================================================
// Represents a room booking in the system
// =============================================================================

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Status of a reservation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReservationStatus {
    Pending,      // Waiting for confirmation
    Confirmed,    // Confirmed and active
    CheckedIn,    // User has arrived
    Completed,    // Meeting finished normally
    Cancelled,    // Cancelled by user or admin
    NoShow,       // User didn't show up
}

impl ReservationStatus {
    pub fn as_str(&self) -> &str {
        match self {
            ReservationStatus::Pending => "Pending",
            ReservationStatus::Confirmed => "Confirmed",
            ReservationStatus::CheckedIn => "Checked In",
            ReservationStatus::Completed => "Completed",
            ReservationStatus::Cancelled => "Cancelled",
            ReservationStatus::NoShow => "No Show",
        }
    }

    pub fn color(&self) -> &str {
        match self {
            ReservationStatus::Pending => "#F59E0B",     // Amber
            ReservationStatus::Confirmed => "#10B981",   // Green
            ReservationStatus::CheckedIn => "#3B82F6",   // Blue
            ReservationStatus::Completed => "#6B7280",   // Gray
            ReservationStatus::Cancelled => "#EF4444",   // Red
            ReservationStatus::NoShow => "#DC2626",      // Dark Red
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "pending" => Some(ReservationStatus::Pending),
            "confirmed" => Some(ReservationStatus::Confirmed),
            "checkedin" | "checked_in" | "checked in" => Some(ReservationStatus::CheckedIn),
            "completed" => Some(ReservationStatus::Completed),
            "cancelled" | "canceled" => Some(ReservationStatus::Cancelled),
            "noshow" | "no_show" | "no show" => Some(ReservationStatus::NoShow),
            _ => None,
        }
    }
}

/// Main Reservation structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reservation {
    pub id: String,
    pub room_id: String,
    pub user_name: String,
    pub user_email: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub attendees: u32,
    pub status: ReservationStatus,
    pub purpose: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub cancelled_at: Option<DateTime<Utc>>,
    pub checked_in_at: Option<DateTime<Utc>>,
    pub recurring_id: Option<String>,  // For recurring reservations
}

impl Reservation {
    pub fn new(
        room_id: String,
        user_name: String,
        user_email: String,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        attendees: u32,
        purpose: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Reservation {
            id: Uuid::new_v4().to_string(),
            room_id,
            user_name,
            user_email,
            start_time,
            end_time,
            attendees,
            status: ReservationStatus::Pending,
            purpose,
            notes: None,
            created_at: now,
            updated_at: now,
            cancelled_at: None,
            checked_in_at: None,
            recurring_id: None,
        }
    }

    /// Duration in minutes
    pub fn duration_minutes(&self) -> i64 {
        (self.end_time - self.start_time).num_minutes()
    }

    /// Duration in hours (rounded)
    pub fn duration_hours(&self) -> f64 {
        self.duration_minutes() as f64 / 60.0
    }

    /// Check if reservation is currently active
    pub fn is_active(&self) -> bool {
        let now = Utc::now();
        self.status == ReservationStatus::Confirmed
            && now >= self.start_time
            && now <= self.end_time
    }

    /// Check if reservation is upcoming
    pub fn is_upcoming(&self) -> bool {
        let now = Utc::now();
        (self.status == ReservationStatus::Pending || self.status == ReservationStatus::Confirmed)
            && now < self.start_time
    }

    /// Check if reservation is in the past
    pub fn is_past(&self) -> bool {
        Utc::now() > self.end_time
    }

    /// Cancel the reservation
    pub fn cancel(&mut self) {
        self.status = ReservationStatus::Cancelled;
        self.cancelled_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }

    /// Confirm the reservation
    pub fn confirm(&mut self) {
        self.status = ReservationStatus::Confirmed;
        self.updated_at = Utc::now();
    }

    /// Check in to the reservation
    pub fn check_in(&mut self) {
        self.status = ReservationStatus::CheckedIn;
        self.checked_in_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }

    /// Complete the reservation
    pub fn complete(&mut self) {
        self.status = ReservationStatus::Completed;
        self.updated_at = Utc::now();
    }

    /// Mark as no-show
    pub fn mark_no_show(&mut self) {
        self.status = ReservationStatus::NoShow;
        self.updated_at = Utc::now();
    }

    /// Get a summary
    pub fn summary(&self) -> ReservationSummary {
        ReservationSummary {
            id: self.id.clone(),
            room_id: self.room_id.clone(),
            user_name: self.user_name.clone(),
            user_email: self.user_email.clone(),
            start_time: self.start_time,
            end_time: self.end_time,
            status: self.status.as_str().to_string(),
            duration_minutes: self.duration_minutes(),
        }
    }

    /// Check for time overlap with another reservation
    pub fn overlaps_with(&self, other: &Reservation) -> bool {
        self.room_id == other.room_id
            && self.id != other.id
            && self.start_time < other.end_time
            && self.end_time > other.start_time
    }
}

/// Lightweight reservation summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReservationSummary {
    pub id: String,
    pub room_id: String,
    pub user_name: String,
    pub user_email: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub status: String,
    pub duration_minutes: i64,
}

/// Request to create a new reservation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateReservationRequest {
    pub room_id: String,
    pub user_name: String,
    pub user_email: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub attendees: u32,
    pub purpose: Option<String>,
    pub notes: Option<String>,
    pub user_role: Option<String>,  // For strategy pattern
}

/// Request to update a reservation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateReservationRequest {
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub attendees: Option<u32>,
    pub purpose: Option<String>,
    pub notes: Option<String>,
}

/// Recurring reservation settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecurringSettings {
    pub pattern: RecurrencePattern,
    pub interval: u32,           // Every N days/weeks/months
    pub end_date: Option<DateTime<Utc>>,
    pub occurrences: Option<u32>, // Or N occurrences
    pub days_of_week: Option<Vec<u8>>, // 0=Sun, 1=Mon, etc.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecurrencePattern {
    Daily,
    Weekly,
    BiWeekly,
    Monthly,
    Custom,
}

/// Filter options for listing reservations
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReservationFilter {
    pub room_id: Option<String>,
    pub user_email: Option<String>,
    pub status: Option<String>,
    pub from_date: Option<DateTime<Utc>>,
    pub to_date: Option<DateTime<Utc>>,
    pub min_attendees: Option<u32>,
    pub max_attendees: Option<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reservation_creation() {
        let now = Utc::now();
        let reservation = Reservation::new(
            "room-1".to_string(),
            "John Doe".to_string(),
            "john@example.com".to_string(),
            now + Duration::hours(1),
            now + Duration::hours(2),
            5,
            Some("Team meeting".to_string()),
        );

        assert_eq!(reservation.status, ReservationStatus::Pending);
        assert_eq!(reservation.duration_minutes(), 60);
        assert!(reservation.is_upcoming());
    }

    #[test]
    fn test_reservation_overlap() {
        let now = Utc::now();
        
        let res1 = Reservation::new(
            "room-1".to_string(),
            "User 1".to_string(),
            "user1@example.com".to_string(),
            now + Duration::hours(1),
            now + Duration::hours(3),
            5,
            None,
        );

        let res2 = Reservation::new(
            "room-1".to_string(),
            "User 2".to_string(),
            "user2@example.com".to_string(),
            now + Duration::hours(2),
            now + Duration::hours(4),
            5,
            None,
        );

        let res3 = Reservation::new(
            "room-1".to_string(),
            "User 3".to_string(),
            "user3@example.com".to_string(),
            now + Duration::hours(4),
            now + Duration::hours(5),
            5,
            None,
        );

        assert!(res1.overlaps_with(&res2)); // Overlapping
        assert!(!res1.overlaps_with(&res3)); // Not overlapping
    }

    #[test]
    fn test_status_transitions() {
        let now = Utc::now();
        let mut reservation = Reservation::new(
            "room-1".to_string(),
            "Test User".to_string(),
            "test@example.com".to_string(),
            now + Duration::hours(1),
            now + Duration::hours(2),
            5,
            None,
        );

        assert_eq!(reservation.status, ReservationStatus::Pending);
        
        reservation.confirm();
        assert_eq!(reservation.status, ReservationStatus::Confirmed);
        
        reservation.check_in();
        assert_eq!(reservation.status, ReservationStatus::CheckedIn);
        assert!(reservation.checked_in_at.is_some());
        
        reservation.complete();
        assert_eq!(reservation.status, ReservationStatus::Completed);
    }
}
