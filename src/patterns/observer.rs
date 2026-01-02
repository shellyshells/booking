// =============================================================================
// OBSERVER PATTERN - Event Notification System
// =============================================================================
// Problem Solved: Decouples event sources from event handlers. When a
//                 reservation is created/modified/cancelled, multiple
//                 observers can be notified (email, logging, analytics, etc.)
// Location: Used by ReservationService to notify interested parties
// =============================================================================

use crate::models::reservation::ReservationStatus;
use crate::patterns::singleton::{log_info, log_warning};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

// -----------------------------------------------------------------------------
// Event Types for the Reservation System
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReservationEvent {
    Created(ReservationEventData),
    Updated(ReservationEventData),
    Cancelled(ReservationEventData),
    Confirmed(ReservationEventData),
    Reminder(ReservationEventData),
    CheckedIn(ReservationEventData),
    CheckedOut(ReservationEventData),
    Conflict(ConflictEventData),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReservationEventData {
    pub reservation_id: String,
    pub room_id: String,
    pub room_name: String,
    pub user_email: String,
    pub user_name: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub event_time: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictEventData {
    pub reservation_id: String,
    pub conflicting_reservation_id: String,
    pub room_id: String,
    pub room_name: String,
    pub event_time: DateTime<Utc>,
}

// -----------------------------------------------------------------------------
// Observer Trait - Interface for event subscribers
// -----------------------------------------------------------------------------
pub trait ReservationObserver: Send + Sync {
    fn on_event(&self, event: &ReservationEvent);
    fn observer_name(&self) -> &str;
    fn observer_id(&self) -> &str;
    fn is_interested_in(&self, event: &ReservationEvent) -> bool;
}

// -----------------------------------------------------------------------------
// Concrete Observer: Email Notifier (Simulated)
// -----------------------------------------------------------------------------
pub struct EmailNotifier {
    id: String,
}

impl EmailNotifier {
    pub fn new() -> Self {
        EmailNotifier {
            id: Uuid::new_v4().to_string(),
        }
    }
}

impl Default for EmailNotifier {
    fn default() -> Self {
        Self::new()
    }
}

impl ReservationObserver for EmailNotifier {
    fn on_event(&self, event: &ReservationEvent) {
        match event {
            ReservationEvent::Created(data) => {
                log_info(
                    &format!(
                        "[EMAIL] Sending confirmation to {} for room {} on {}",
                        data.user_email,
                        data.room_name,
                        data.start_time.format("%Y-%m-%d %H:%M")
                    ),
                    Some("EmailNotifier"),
                );
            }
            ReservationEvent::Cancelled(data) => {
                log_info(
                    &format!(
                        "[EMAIL] Sending cancellation notice to {} for room {}",
                        data.user_email, data.room_name
                    ),
                    Some("EmailNotifier"),
                );
            }
            ReservationEvent::Reminder(data) => {
                log_info(
                    &format!(
                        "[EMAIL] Sending reminder to {} - {} starts at {}",
                        data.user_email,
                        data.room_name,
                        data.start_time.format("%H:%M")
                    ),
                    Some("EmailNotifier"),
                );
            }
            ReservationEvent::Updated(data) => {
                log_info(
                    &format!(
                        "[EMAIL] Sending update notification to {} for room {}",
                        data.user_email, data.room_name
                    ),
                    Some("EmailNotifier"),
                );
            }
            _ => {}
        }
    }

    fn observer_name(&self) -> &str {
        "Email Notifier"
    }

    fn observer_id(&self) -> &str {
        &self.id
    }

    fn is_interested_in(&self, event: &ReservationEvent) -> bool {
        matches!(
            event,
            ReservationEvent::Created(_)
                | ReservationEvent::Cancelled(_)
                | ReservationEvent::Reminder(_)
                | ReservationEvent::Updated(_)
        )
    }
}

// -----------------------------------------------------------------------------
// Concrete Observer: Analytics Tracker
// -----------------------------------------------------------------------------
pub struct AnalyticsTracker {
    id: String,
    events_tracked: RwLock<Vec<TrackedEvent>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackedEvent {
    pub event_type: String,
    pub timestamp: DateTime<Utc>,
    pub data: HashMap<String, String>,
}

impl AnalyticsTracker {
    pub fn new() -> Self {
        AnalyticsTracker {
            id: Uuid::new_v4().to_string(),
            events_tracked: RwLock::new(Vec::new()),
        }
    }

    pub fn get_tracked_events(&self) -> Vec<TrackedEvent> {
        self.events_tracked.read().unwrap().clone()
    }

    pub fn get_event_count(&self) -> usize {
        self.events_tracked.read().unwrap().len()
    }

    fn track(&self, event_type: &str, data: HashMap<String, String>) {
        let tracked = TrackedEvent {
            event_type: event_type.to_string(),
            timestamp: Utc::now(),
            data,
        };
        
        let mut events = self.events_tracked.write().unwrap();
        events.push(tracked);
        
        log_info(
            &format!("[ANALYTICS] Tracked event: {} (total: {})", event_type, events.len()),
            Some("AnalyticsTracker"),
        );
    }
}

impl Default for AnalyticsTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl ReservationObserver for AnalyticsTracker {
    fn on_event(&self, event: &ReservationEvent) {
        let (event_type, data) = match event {
            ReservationEvent::Created(d) => {
                let mut map = HashMap::new();
                map.insert("room_id".to_string(), d.room_id.clone());
                map.insert("user_email".to_string(), d.user_email.clone());
                ("reservation_created", map)
            }
            ReservationEvent::Cancelled(d) => {
                let mut map = HashMap::new();
                map.insert("room_id".to_string(), d.room_id.clone());
                ("reservation_cancelled", map)
            }
            ReservationEvent::Confirmed(d) => {
                let mut map = HashMap::new();
                map.insert("room_id".to_string(), d.room_id.clone());
                ("reservation_confirmed", map)
            }
            ReservationEvent::CheckedIn(d) => {
                let mut map = HashMap::new();
                map.insert("room_id".to_string(), d.room_id.clone());
                ("check_in", map)
            }
            ReservationEvent::CheckedOut(d) => {
                let mut map = HashMap::new();
                map.insert("room_id".to_string(), d.room_id.clone());
                ("check_out", map)
            }
            ReservationEvent::Conflict(d) => {
                let mut map = HashMap::new();
                map.insert("room_id".to_string(), d.room_id.clone());
                ("conflict_detected", map)
            }
            _ => return,
        };

        self.track(event_type, data);
    }

    fn observer_name(&self) -> &str {
        "Analytics Tracker"
    }

    fn observer_id(&self) -> &str {
        &self.id
    }

    fn is_interested_in(&self, _event: &ReservationEvent) -> bool {
        true
    }
}

// -----------------------------------------------------------------------------
// Concrete Observer: Audit Logger
// -----------------------------------------------------------------------------
pub struct AuditLogger {
    id: String,
    audit_log: RwLock<Vec<AuditEntry>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub action: String,
    pub entity_type: String,
    pub entity_id: String,
    pub user: String,
    pub details: String,
}

impl AuditLogger {
    pub fn new() -> Self {
        AuditLogger {
            id: Uuid::new_v4().to_string(),
            audit_log: RwLock::new(Vec::new()),
        }
    }

    pub fn get_audit_log(&self) -> Vec<AuditEntry> {
        self.audit_log.read().unwrap().clone()
    }

    fn log_audit(&self, action: &str, entity_id: &str, user: &str, details: &str) {
        let entry = AuditEntry {
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            action: action.to_string(),
            entity_type: "Reservation".to_string(),
            entity_id: entity_id.to_string(),
            user: user.to_string(),
            details: details.to_string(),
        };

        let mut log = self.audit_log.write().unwrap();
        log.push(entry);

        log_info(
            &format!("[AUDIT] {} - {} by {}", action, entity_id, user),
            Some("AuditLogger"),
        );
    }
}

impl Default for AuditLogger {
    fn default() -> Self {
        Self::new()
    }
}

impl ReservationObserver for AuditLogger {
    fn on_event(&self, event: &ReservationEvent) {
        match event {
            ReservationEvent::Created(d) => {
                self.log_audit("CREATE", &d.reservation_id, &d.user_email,
                    &format!("Created for {}", d.room_name));
            }
            ReservationEvent::Updated(d) => {
                self.log_audit("UPDATE", &d.reservation_id, &d.user_email,
                    &format!("Updated for {}", d.room_name));
            }
            ReservationEvent::Cancelled(d) => {
                self.log_audit("CANCEL", &d.reservation_id, &d.user_email,
                    &format!("Cancelled for {}", d.room_name));
            }
            ReservationEvent::Confirmed(d) => {
                self.log_audit("CONFIRM", &d.reservation_id, &d.user_email,
                    &format!("Confirmed for {}", d.room_name));
            }
            _ => {}
        }
    }

    fn observer_name(&self) -> &str {
        "Audit Logger"
    }

    fn observer_id(&self) -> &str {
        &self.id
    }

    fn is_interested_in(&self, _event: &ReservationEvent) -> bool {
        true
    }
}

// -----------------------------------------------------------------------------
// Concrete Observer: Slack Notifier (Simulated)
// -----------------------------------------------------------------------------
pub struct SlackNotifier {
    id: String,
    channel: String,
}

impl SlackNotifier {
    pub fn new(channel: &str) -> Self {
        SlackNotifier {
            id: Uuid::new_v4().to_string(),
            channel: channel.to_string(),
        }
    }
}

impl ReservationObserver for SlackNotifier {
    fn on_event(&self, event: &ReservationEvent) {
        match event {
            ReservationEvent::Created(d) => {
                log_info(
                    &format!(
                        "[SLACK #{}] New booking: {} reserved {}",
                        self.channel, d.user_name, d.room_name
                    ),
                    Some("SlackNotifier"),
                );
            }
            ReservationEvent::Cancelled(d) => {
                log_info(
                    &format!(
                        "[SLACK #{}] Cancelled: {} cancelled {}",
                        self.channel, d.user_name, d.room_name
                    ),
                    Some("SlackNotifier"),
                );
            }
            ReservationEvent::Conflict(d) => {
                log_warning(
                    &format!("[SLACK #{}] Conflict alert in {}!", self.channel, d.room_name),
                    Some("SlackNotifier"),
                );
            }
            _ => {}
        }
    }

    fn observer_name(&self) -> &str {
        "Slack Notifier"
    }

    fn observer_id(&self) -> &str {
        &self.id
    }

    fn is_interested_in(&self, event: &ReservationEvent) -> bool {
        matches!(
            event,
            ReservationEvent::Created(_)
                | ReservationEvent::Cancelled(_)
                | ReservationEvent::Conflict(_)
        )
    }
}

// -----------------------------------------------------------------------------
// Event Publisher - Subject that manages observers
// -----------------------------------------------------------------------------
pub struct EventPublisher {
    observers: RwLock<Vec<Arc<dyn ReservationObserver>>>,
}

impl EventPublisher {
    pub fn new() -> Self {
        log_info("Created new EventPublisher", Some("EventPublisher"));
        EventPublisher {
            observers: RwLock::new(Vec::new()),
        }
    }

    pub fn subscribe(&self, observer: Arc<dyn ReservationObserver>) {
        let name = observer.observer_name().to_string();
        let id = observer.observer_id().to_string();
        
        let mut observers = self.observers.write().unwrap();
        
        if observers.iter().any(|o| o.observer_id() == id) {
            log_warning(&format!("Observer {} already subscribed", name), Some("EventPublisher"));
            return;
        }
        
        observers.push(observer);
        log_info(&format!("Subscribed: {} (total: {})", name, observers.len()), Some("EventPublisher"));
    }

    pub fn unsubscribe(&self, observer_id: &str) {
        let mut observers = self.observers.write().unwrap();
        if let Some(pos) = observers.iter().position(|o| o.observer_id() == observer_id) {
            let name = observers[pos].observer_name().to_string();
            observers.remove(pos);
            log_info(&format!("Unsubscribed: {}", name), Some("EventPublisher"));
        }
    }

    pub fn publish(&self, event: ReservationEvent) {
        let observers = self.observers.read().unwrap();
        let mut notified = 0;

        for observer in observers.iter() {
            if observer.is_interested_in(&event) {
                observer.on_event(&event);
                notified += 1;
            }
        }

        log_info(&format!("Published to {} observers", notified), Some("EventPublisher"));
    }

    pub fn list_observers(&self) -> Vec<ObserverInfo> {
        self.observers.read().unwrap().iter()
            .map(|o| ObserverInfo {
                id: o.observer_id().to_string(),
                name: o.observer_name().to_string(),
            })
            .collect()
    }

    pub fn observer_count(&self) -> usize {
        self.observers.read().unwrap().len()
    }
}

impl Default for EventPublisher {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObserverInfo {
    pub id: String,
    pub name: String,
}

pub fn create_default_observers() -> Vec<Arc<dyn ReservationObserver>> {
    vec![
        Arc::new(EmailNotifier::new()),
        Arc::new(AnalyticsTracker::new()),
        Arc::new(AuditLogger::new()),
        Arc::new(SlackNotifier::new("room-bookings")),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_observer_subscription() {
        let publisher = EventPublisher::new();
        let observer = Arc::new(EmailNotifier::new());
        
        publisher.subscribe(observer);
        assert_eq!(publisher.observer_count(), 1);
    }
}
