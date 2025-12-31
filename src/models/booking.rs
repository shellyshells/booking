use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BookingStatus {
    Pending,
    Confirmed,
    Cancelled,
    Completed,
}

impl std::fmt::Display for BookingStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BookingStatus::Pending => write!(f, "Pending"),
            BookingStatus::Confirmed => write!(f, "Confirmed"),
            BookingStatus::Cancelled => write!(f, "Cancelled"),
            BookingStatus::Completed => write!(f, "Completed"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Booking {
    pub id: Uuid,
    pub room_id: Uuid,
    pub user_id: Uuid,
    pub start_time: DateTime<Local>,
    pub end_time: DateTime<Local>,
    pub status: BookingStatus,
    pub purpose: String,
    pub attendees: u32,
    pub total_cost: f64,
}

impl Booking {
    pub fn new(
        room_id: Uuid,
        user_id: Uuid,
        start_time: DateTime<Local>,
        end_time: DateTime<Local>,
        purpose: String,
        attendees: u32,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            room_id,
            user_id,
            start_time,
            end_time,
            status: BookingStatus::Pending,
            purpose,
            attendees,
            total_cost: 0.0,
        }
    }

    pub fn duration_hours(&self) -> f64 {
        let duration = self.end_time.signed_duration_since(self.start_time);
        duration.num_minutes() as f64 / 60.0
    }

    pub fn calculate_cost(&mut self, hourly_rate: f64) {
        self.total_cost = self.duration_hours() * hourly_rate;
    }

    pub fn confirm(&mut self) {
        self.status = BookingStatus::Confirmed;
    }

    pub fn cancel(&mut self) {
        self.status = BookingStatus::Cancelled;
    }

    pub fn complete(&mut self) {
        self.status = BookingStatus::Completed;
    }

    pub fn overlaps_with(&self, other: &Booking) -> bool {
        if self.room_id != other.room_id {
            return false;
        }
        
        self.start_time < other.end_time && self.end_time > other.start_time
    }
}
