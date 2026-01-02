// =============================================================================
// ADAPTER PATTERN - External System Integration
// =============================================================================
// Problem Solved: Allows integration with external APIs (calendars, storage)
//                 without changing the core application. Different calendar
//                 systems (Google, Outlook, iCal) can be adapted to work
//                 with our reservation system's interface.
// Location: Used for calendar sync and external storage features
// =============================================================================

use crate::models::reservation::Reservation;
use crate::patterns::singleton::log_info;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// -----------------------------------------------------------------------------
// Internal Calendar Event Format
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarEvent {
    pub id: String,
    pub title: String,
    pub description: String,
    pub location: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub organizer: String,
    pub attendees: Vec<String>,
    pub is_all_day: bool,
    pub recurrence: Option<String>,
    pub metadata: HashMap<String, String>,
}

impl CalendarEvent {
    /// Create a calendar event from a reservation
    pub fn from_reservation(reservation: &Reservation, room_name: &str) -> Self {
        let mut metadata = HashMap::new();
        metadata.insert("reservation_id".to_string(), reservation.id.clone());
        metadata.insert("room_id".to_string(), reservation.room_id.clone());
        metadata.insert("source".to_string(), "room_reservation_system".to_string());

        CalendarEvent {
            id: format!("res_{}", reservation.id),
            title: format!("Room Booking: {}", room_name),
            description: reservation.purpose.clone().unwrap_or_default(),
            location: room_name.to_string(),
            start_time: reservation.start_time,
            end_time: reservation.end_time,
            organizer: reservation.user_email.clone(),
            attendees: vec![reservation.user_email.clone()],
            is_all_day: false,
            recurrence: None,
            metadata,
        }
    }
}

// -----------------------------------------------------------------------------
// Calendar Adapter Trait - Target interface
// -----------------------------------------------------------------------------
pub trait CalendarAdapter: Send + Sync {
    fn create_event(&self, event: &CalendarEvent) -> Result<String, String>;
    fn update_event(&self, event_id: &str, event: &CalendarEvent) -> Result<(), String>;
    fn delete_event(&self, event_id: &str) -> Result<(), String>;
    fn get_events(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Vec<CalendarEvent>, String>;
    fn adapter_name(&self) -> &str;
    fn is_connected(&self) -> bool;
}

// -----------------------------------------------------------------------------
// Google Calendar Adapter (Simulated)
// -----------------------------------------------------------------------------
pub struct GoogleCalendarAdapter {
    api_key: String,
    calendar_id: String,
    connected: bool,
}

impl GoogleCalendarAdapter {
    pub fn new(api_key: &str, calendar_id: &str) -> Self {
        log_info(
            &format!("Initializing Google Calendar adapter for calendar: {}", calendar_id),
            Some("GoogleCalendarAdapter"),
        );
        GoogleCalendarAdapter {
            api_key: api_key.to_string(),
            calendar_id: calendar_id.to_string(),
            connected: true, // Simulated connection
        }
    }

    /// Simulated Google Calendar API format conversion
    fn to_google_format(&self, event: &CalendarEvent) -> GoogleCalendarEvent {
        GoogleCalendarEvent {
            kind: "calendar#event".to_string(),
            etag: format!("\"{}\"", uuid::Uuid::new_v4()),
            id: event.id.clone(),
            status: "confirmed".to_string(),
            summary: event.title.clone(),
            description: event.description.clone(),
            location: event.location.clone(),
            start: GoogleDateTime {
                date_time: event.start_time.to_rfc3339(),
                time_zone: "UTC".to_string(),
            },
            end: GoogleDateTime {
                date_time: event.end_time.to_rfc3339(),
                time_zone: "UTC".to_string(),
            },
            organizer: GoogleAttendee {
                email: event.organizer.clone(),
                display_name: None,
                response_status: "accepted".to_string(),
            },
            attendees: event.attendees.iter().map(|a| GoogleAttendee {
                email: a.clone(),
                display_name: None,
                response_status: "needsAction".to_string(),
            }).collect(),
        }
    }
}

// Simulated Google Calendar event format
#[derive(Debug, Clone, Serialize, Deserialize)]
struct GoogleCalendarEvent {
    kind: String,
    etag: String,
    id: String,
    status: String,
    summary: String,
    description: String,
    location: String,
    start: GoogleDateTime,
    end: GoogleDateTime,
    organizer: GoogleAttendee,
    attendees: Vec<GoogleAttendee>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GoogleDateTime {
    date_time: String,
    time_zone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GoogleAttendee {
    email: String,
    display_name: Option<String>,
    response_status: String,
}

impl CalendarAdapter for GoogleCalendarAdapter {
    fn create_event(&self, event: &CalendarEvent) -> Result<String, String> {
        if !self.connected {
            return Err("Not connected to Google Calendar".to_string());
        }

        let google_event = self.to_google_format(event);
        log_info(
            &format!(
                "[GOOGLE API] POST /calendars/{}/events - Creating: {}",
                self.calendar_id, google_event.summary
            ),
            Some("GoogleCalendarAdapter"),
        );

        // Simulate successful API call
        Ok(google_event.id)
    }

    fn update_event(&self, event_id: &str, event: &CalendarEvent) -> Result<(), String> {
        if !self.connected {
            return Err("Not connected to Google Calendar".to_string());
        }

        let google_event = self.to_google_format(event);
        log_info(
            &format!(
                "[GOOGLE API] PUT /calendars/{}/events/{} - Updating: {}",
                self.calendar_id, event_id, google_event.summary
            ),
            Some("GoogleCalendarAdapter"),
        );

        Ok(())
    }

    fn delete_event(&self, event_id: &str) -> Result<(), String> {
        if !self.connected {
            return Err("Not connected to Google Calendar".to_string());
        }

        log_info(
            &format!(
                "[GOOGLE API] DELETE /calendars/{}/events/{}",
                self.calendar_id, event_id
            ),
            Some("GoogleCalendarAdapter"),
        );

        Ok(())
    }

    fn get_events(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Vec<CalendarEvent>, String> {
        if !self.connected {
            return Err("Not connected to Google Calendar".to_string());
        }

        log_info(
            &format!(
                "[GOOGLE API] GET /calendars/{}/events?timeMin={}&timeMax={}",
                self.calendar_id,
                start.to_rfc3339(),
                end.to_rfc3339()
            ),
            Some("GoogleCalendarAdapter"),
        );

        // Return empty list (simulated)
        Ok(vec![])
    }

    fn adapter_name(&self) -> &str {
        "Google Calendar"
    }

    fn is_connected(&self) -> bool {
        self.connected
    }
}

// -----------------------------------------------------------------------------
// Outlook Calendar Adapter (Simulated)
// -----------------------------------------------------------------------------
pub struct OutlookCalendarAdapter {
    client_id: String,
    user_id: String,
    connected: bool,
}

impl OutlookCalendarAdapter {
    pub fn new(client_id: &str, user_id: &str) -> Self {
        log_info(
            &format!("Initializing Outlook Calendar adapter for user: {}", user_id),
            Some("OutlookCalendarAdapter"),
        );
        OutlookCalendarAdapter {
            client_id: client_id.to_string(),
            user_id: user_id.to_string(),
            connected: true,
        }
    }

    /// Convert to Microsoft Graph API format
    fn to_outlook_format(&self, event: &CalendarEvent) -> OutlookEvent {
        OutlookEvent {
            odata_type: "#microsoft.graph.event".to_string(),
            id: event.id.clone(),
            subject: event.title.clone(),
            body: OutlookBody {
                content_type: "text".to_string(),
                content: event.description.clone(),
            },
            start: OutlookDateTime {
                date_time: event.start_time.format("%Y-%m-%dT%H:%M:%S").to_string(),
                time_zone: "UTC".to_string(),
            },
            end: OutlookDateTime {
                date_time: event.end_time.format("%Y-%m-%dT%H:%M:%S").to_string(),
                time_zone: "UTC".to_string(),
            },
            location: OutlookLocation {
                display_name: event.location.clone(),
            },
            attendees: event.attendees.iter().map(|a| OutlookAttendee {
                email_address: OutlookEmailAddress {
                    address: a.clone(),
                    name: a.clone(),
                },
                attendee_type: "required".to_string(),
            }).collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OutlookEvent {
    #[serde(rename = "@odata.type")]
    odata_type: String,
    id: String,
    subject: String,
    body: OutlookBody,
    start: OutlookDateTime,
    end: OutlookDateTime,
    location: OutlookLocation,
    attendees: Vec<OutlookAttendee>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OutlookBody {
    content_type: String,
    content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OutlookDateTime {
    date_time: String,
    time_zone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OutlookLocation {
    display_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OutlookAttendee {
    email_address: OutlookEmailAddress,
    attendee_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OutlookEmailAddress {
    address: String,
    name: String,
}

impl CalendarAdapter for OutlookCalendarAdapter {
    fn create_event(&self, event: &CalendarEvent) -> Result<String, String> {
        if !self.connected {
            return Err("Not connected to Outlook Calendar".to_string());
        }

        let outlook_event = self.to_outlook_format(event);
        log_info(
            &format!(
                "[GRAPH API] POST /users/{}/events - Creating: {}",
                self.user_id, outlook_event.subject
            ),
            Some("OutlookCalendarAdapter"),
        );

        Ok(outlook_event.id)
    }

    fn update_event(&self, event_id: &str, event: &CalendarEvent) -> Result<(), String> {
        if !self.connected {
            return Err("Not connected to Outlook Calendar".to_string());
        }

        let outlook_event = self.to_outlook_format(event);
        log_info(
            &format!(
                "[GRAPH API] PATCH /users/{}/events/{} - Updating: {}",
                self.user_id, event_id, outlook_event.subject
            ),
            Some("OutlookCalendarAdapter"),
        );

        Ok(())
    }

    fn delete_event(&self, event_id: &str) -> Result<(), String> {
        if !self.connected {
            return Err("Not connected to Outlook Calendar".to_string());
        }

        log_info(
            &format!(
                "[GRAPH API] DELETE /users/{}/events/{}",
                self.user_id, event_id
            ),
            Some("OutlookCalendarAdapter"),
        );

        Ok(())
    }

    fn get_events(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Vec<CalendarEvent>, String> {
        if !self.connected {
            return Err("Not connected to Outlook Calendar".to_string());
        }

        log_info(
            &format!(
                "[GRAPH API] GET /users/{}/calendarView?startDateTime={}&endDateTime={}",
                self.user_id,
                start.to_rfc3339(),
                end.to_rfc3339()
            ),
            Some("OutlookCalendarAdapter"),
        );

        Ok(vec![])
    }

    fn adapter_name(&self) -> &str {
        "Microsoft Outlook"
    }

    fn is_connected(&self) -> bool {
        self.connected
    }
}

// -----------------------------------------------------------------------------
// iCal Format Adapter (for .ics file export)
// -----------------------------------------------------------------------------
pub struct ICalAdapter {
    output_path: String,
}

impl ICalAdapter {
    pub fn new(output_path: &str) -> Self {
        log_info(
            &format!("Initializing iCal adapter, output: {}", output_path),
            Some("ICalAdapter"),
        );
        ICalAdapter {
            output_path: output_path.to_string(),
        }
    }

    /// Generate iCal format string
    pub fn to_ical_format(&self, event: &CalendarEvent) -> String {
        let uid = format!("{}@roomreservation.local", event.id);
        let dtstamp = Utc::now().format("%Y%m%dT%H%M%SZ").to_string();
        let dtstart = event.start_time.format("%Y%m%dT%H%M%SZ").to_string();
        let dtend = event.end_time.format("%Y%m%dT%H%M%SZ").to_string();

        format!(
            "BEGIN:VCALENDAR\r\n\
            VERSION:2.0\r\n\
            PRODID:-//Room Reservation System//EN\r\n\
            CALSCALE:GREGORIAN\r\n\
            METHOD:PUBLISH\r\n\
            BEGIN:VEVENT\r\n\
            UID:{}\r\n\
            DTSTAMP:{}\r\n\
            DTSTART:{}\r\n\
            DTEND:{}\r\n\
            SUMMARY:{}\r\n\
            DESCRIPTION:{}\r\n\
            LOCATION:{}\r\n\
            ORGANIZER:mailto:{}\r\n\
            STATUS:CONFIRMED\r\n\
            END:VEVENT\r\n\
            END:VCALENDAR\r\n",
            uid, dtstamp, dtstart, dtend, 
            event.title, event.description, event.location, event.organizer
        )
    }

    /// Generate iCal for multiple events
    pub fn to_ical_batch(&self, events: &[CalendarEvent]) -> String {
        let mut ical = String::from(
            "BEGIN:VCALENDAR\r\n\
            VERSION:2.0\r\n\
            PRODID:-//Room Reservation System//EN\r\n\
            CALSCALE:GREGORIAN\r\n\
            METHOD:PUBLISH\r\n"
        );

        for event in events {
            let uid = format!("{}@roomreservation.local", event.id);
            let dtstamp = Utc::now().format("%Y%m%dT%H%M%SZ").to_string();
            let dtstart = event.start_time.format("%Y%m%dT%H%M%SZ").to_string();
            let dtend = event.end_time.format("%Y%m%dT%H%M%SZ").to_string();

            ical.push_str(&format!(
                "BEGIN:VEVENT\r\n\
                UID:{}\r\n\
                DTSTAMP:{}\r\n\
                DTSTART:{}\r\n\
                DTEND:{}\r\n\
                SUMMARY:{}\r\n\
                DESCRIPTION:{}\r\n\
                LOCATION:{}\r\n\
                END:VEVENT\r\n",
                uid, dtstamp, dtstart, dtend,
                event.title, event.description, event.location
            ));
        }

        ical.push_str("END:VCALENDAR\r\n");
        ical
    }
}

impl CalendarAdapter for ICalAdapter {
    fn create_event(&self, event: &CalendarEvent) -> Result<String, String> {
        let ical_content = self.to_ical_format(event);
        log_info(
            &format!("[ICAL] Generated .ics file for: {}", event.title),
            Some("ICalAdapter"),
        );
        
        // In real implementation, would write to file
        // std::fs::write(&self.output_path, &ical_content)?;
        
        Ok(event.id.clone())
    }

    fn update_event(&self, event_id: &str, event: &CalendarEvent) -> Result<(), String> {
        log_info(
            &format!("[ICAL] Updated event {} in .ics", event_id),
            Some("ICalAdapter"),
        );
        Ok(())
    }

    fn delete_event(&self, event_id: &str) -> Result<(), String> {
        log_info(
            &format!("[ICAL] Removed event {} from .ics", event_id),
            Some("ICalAdapter"),
        );
        Ok(())
    }

    fn get_events(&self, _start: DateTime<Utc>, _end: DateTime<Utc>) -> Result<Vec<CalendarEvent>, String> {
        log_info("[ICAL] Reading events from .ics file", Some("ICalAdapter"));
        Ok(vec![])
    }

    fn adapter_name(&self) -> &str {
        "iCal Export"
    }

    fn is_connected(&self) -> bool {
        true
    }
}

// -----------------------------------------------------------------------------
// Calendar Service - Uses adapters to sync with external calendars
// -----------------------------------------------------------------------------
pub struct CalendarService {
    adapters: Vec<Box<dyn CalendarAdapter>>,
}

impl CalendarService {
    pub fn new() -> Self {
        CalendarService {
            adapters: Vec::new(),
        }
    }

    pub fn add_adapter(&mut self, adapter: Box<dyn CalendarAdapter>) {
        log_info(
            &format!("Added calendar adapter: {}", adapter.adapter_name()),
            Some("CalendarService"),
        );
        self.adapters.push(adapter);
    }

    pub fn sync_reservation(&self, reservation: &Reservation, room_name: &str) -> Vec<SyncResult> {
        let event = CalendarEvent::from_reservation(reservation, room_name);
        let mut results = Vec::new();

        for adapter in &self.adapters {
            let result = match adapter.create_event(&event) {
                Ok(id) => SyncResult {
                    adapter_name: adapter.adapter_name().to_string(),
                    success: true,
                    external_id: Some(id),
                    error: None,
                },
                Err(e) => SyncResult {
                    adapter_name: adapter.adapter_name().to_string(),
                    success: false,
                    external_id: None,
                    error: Some(e),
                },
            };
            results.push(result);
        }

        results
    }

    pub fn list_adapters(&self) -> Vec<AdapterInfo> {
        self.adapters.iter().map(|a| AdapterInfo {
            name: a.adapter_name().to_string(),
            connected: a.is_connected(),
        }).collect()
    }
}

impl Default for CalendarService {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncResult {
    pub adapter_name: String,
    pub success: bool,
    pub external_id: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterInfo {
    pub name: String,
    pub connected: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_google_adapter() {
        let adapter = GoogleCalendarAdapter::new("test-api-key", "test@calendar.google.com");
        assert!(adapter.is_connected());
        assert_eq!(adapter.adapter_name(), "Google Calendar");
    }

    #[test]
    fn test_ical_generation() {
        let adapter = ICalAdapter::new("/tmp/test.ics");
        let event = CalendarEvent {
            id: "test-123".to_string(),
            title: "Test Meeting".to_string(),
            description: "A test meeting".to_string(),
            location: "Room A".to_string(),
            start_time: Utc::now(),
            end_time: Utc::now() + chrono::Duration::hours(1),
            organizer: "test@example.com".to_string(),
            attendees: vec!["test@example.com".to_string()],
            is_all_day: false,
            recurrence: None,
            metadata: HashMap::new(),
        };

        let ical = adapter.to_ical_format(&event);
        assert!(ical.contains("BEGIN:VCALENDAR"));
        assert!(ical.contains("Test Meeting"));
        assert!(ical.contains("END:VCALENDAR"));
    }

    #[test]
    fn test_calendar_service() {
        let mut service = CalendarService::new();
        service.add_adapter(Box::new(GoogleCalendarAdapter::new("key", "cal")));
        service.add_adapter(Box::new(OutlookCalendarAdapter::new("client", "user")));

        let adapters = service.list_adapters();
        assert_eq!(adapters.len(), 2);
    }
}
