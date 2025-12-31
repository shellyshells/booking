use crate::models::Booking;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Adapter Pattern - Integrate with external calendar system

/// External calendar system interface (simulated)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalCalendarEvent {
    pub event_id: String,
    pub title: String,
    pub start: String, // RFC3339 format
    pub end: String,
    pub location: String,
    pub attendees: Vec<String>,
}

pub trait ExternalCalendarSystem {
    fn create_event(&self, event: ExternalCalendarEvent) -> Result<String, String>;
    fn get_event(&self, event_id: &str) -> Result<ExternalCalendarEvent, String>;
    fn update_event(&self, event: ExternalCalendarEvent) -> Result<(), String>;
    fn delete_event(&self, event_id: &str) -> Result<(), String>;
}

/// Simulated Google Calendar API
pub struct GoogleCalendarAPI;

impl ExternalCalendarSystem for GoogleCalendarAPI {
    fn create_event(&self, event: ExternalCalendarEvent) -> Result<String, String> {
        // Simulate API call
        Ok(format!("gcal_{}", Uuid::new_v4()))
    }

    fn get_event(&self, _event_id: &str) -> Result<ExternalCalendarEvent, String> {
        // Simulate API call
        Err("Not implemented in simulation".to_string())
    }

    fn update_event(&self, _event: ExternalCalendarEvent) -> Result<(), String> {
        Ok(())
    }

    fn delete_event(&self, _event_id: &str) -> Result<(), String> {
        Ok(())
    }
}

/// Adapter that converts our Booking to ExternalCalendarEvent
pub struct CalendarAdapter<T: ExternalCalendarSystem> {
    calendar: T,
}

impl<T: ExternalCalendarSystem> CalendarAdapter<T> {
    pub fn new(calendar: T) -> Self {
        Self { calendar }
    }

    fn booking_to_event(&self, booking: &Booking, room_name: &str) -> ExternalCalendarEvent {
        ExternalCalendarEvent {
            event_id: booking.id.to_string(),
            title: format!("{} - {}", room_name, booking.purpose),
            start: booking.start_time.to_rfc3339(),
            end: booking.end_time.to_rfc3339(),
            location: room_name.to_string(),
            attendees: vec![booking.user_id.to_string()],
        }
    }

    pub fn sync_booking(&self, booking: &Booking, room_name: &str) -> Result<String, String> {
        let event = self.booking_to_event(booking, room_name);
        self.calendar.create_event(event)
    }

    pub fn cancel_booking(&self, booking_id: &str) -> Result<(), String> {
        self.calendar.delete_event(booking_id)
    }

    pub fn update_booking(&self, booking: &Booking, room_name: &str) -> Result<(), String> {
        let event = self.booking_to_event(booking, room_name);
        self.calendar.update_event(event)
    }
}

/// Simulated Outlook Calendar API
pub struct OutlookCalendarAPI;

impl ExternalCalendarSystem for OutlookCalendarAPI {
    fn create_event(&self, event: ExternalCalendarEvent) -> Result<String, String> {
        Ok(format!("outlook_{}", Uuid::new_v4()))
    }

    fn get_event(&self, _event_id: &str) -> Result<ExternalCalendarEvent, String> {
        Err("Not implemented in simulation".to_string())
    }

    fn update_event(&self, _event: ExternalCalendarEvent) -> Result<(), String> {
        Ok(())
    }

    fn delete_event(&self, _event_id: &str) -> Result<(), String> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_google_calendar_adapter() {
        let adapter = CalendarAdapter::new(GoogleCalendarAPI);
        
        let booking = Booking::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            Local::now(),
            Local::now() + Duration::hours(2),
            "Team Meeting".to_string(),
            5,
        );

        let result = adapter.sync_booking(&booking, "Conference Room A");
        assert!(result.is_ok());
        assert!(result.unwrap().starts_with("gcal_"));
    }

    #[test]
    fn test_outlook_calendar_adapter() {
        let adapter = CalendarAdapter::new(OutlookCalendarAPI);
        
        let booking = Booking::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            Local::now(),
            Local::now() + Duration::hours(1),
            "Client Call".to_string(),
            2,
        );

        let result = adapter.sync_booking(&booking, "Meeting Room B");
        assert!(result.is_ok());
        assert!(result.unwrap().starts_with("outlook_"));
    }
}
