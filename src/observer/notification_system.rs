use crate::models::{Booking, BookingStatus};
use std::sync::Arc;
use parking_lot::RwLock;

/// Observer Pattern - Notification system for booking events
pub trait BookingObserver: Send + Sync {
    fn on_booking_created(&self, booking: &Booking);
    fn on_booking_confirmed(&self, booking: &Booking);
    fn on_booking_cancelled(&self, booking: &Booking);
    fn on_booking_completed(&self, booking: &Booking);
}

/// Email notification observer
pub struct EmailNotifier {
    enabled: bool,
}

impl EmailNotifier {
    pub fn new() -> Self {
        Self { enabled: true }
    }
}

impl BookingObserver for EmailNotifier {
    fn on_booking_created(&self, booking: &Booking) {
        if self.enabled {
            println!("📧 Email: New booking created - ID: {}", booking.id);
        }
    }

    fn on_booking_confirmed(&self, booking: &Booking) {
        if self.enabled {
            println!("📧 Email: Booking confirmed - ID: {}", booking.id);
        }
    }

    fn on_booking_cancelled(&self, booking: &Booking) {
        if self.enabled {
            println!("📧 Email: Booking cancelled - ID: {}", booking.id);
        }
    }

    fn on_booking_completed(&self, booking: &Booking) {
        if self.enabled {
            println!("📧 Email: Booking completed - ID: {}", booking.id);
        }
    }
}

/// SMS notification observer
pub struct SMSNotifier {
    enabled: bool,
}

impl SMSNotifier {
    pub fn new() -> Self {
        Self { enabled: true }
    }
}

impl BookingObserver for SMSNotifier {
    fn on_booking_created(&self, booking: &Booking) {
        if self.enabled {
            println!("📱 SMS: New booking - ID: {}", booking.id);
        }
    }

    fn on_booking_confirmed(&self, booking: &Booking) {
        if self.enabled {
            println!("📱 SMS: Confirmed - ID: {}", booking.id);
        }
    }

    fn on_booking_cancelled(&self, booking: &Booking) {
        if self.enabled {
            println!("📱 SMS: Cancelled - ID: {}", booking.id);
        }
    }

    fn on_booking_completed(&self, _booking: &Booking) {
        // No SMS for completion
    }
}

/// Audit log observer
pub struct AuditLogger;

impl AuditLogger {
    pub fn new() -> Self {
        Self
    }
}

impl BookingObserver for AuditLogger {
    fn on_booking_created(&self, booking: &Booking) {
        crate::logger::Logger::instance().info(&format!(
            "AUDIT: Booking created - ID: {}, User: {}, Room: {}",
            booking.id, booking.user_id, booking.room_id
        ));
    }

    fn on_booking_confirmed(&self, booking: &Booking) {
        crate::logger::Logger::instance().info(&format!(
            "AUDIT: Booking confirmed - ID: {}",
            booking.id
        ));
    }

    fn on_booking_cancelled(&self, booking: &Booking) {
        crate::logger::Logger::instance().warn(&format!(
            "AUDIT: Booking cancelled - ID: {}",
            booking.id
        ));
    }

    fn on_booking_completed(&self, booking: &Booking) {
        crate::logger::Logger::instance().info(&format!(
            "AUDIT: Booking completed - ID: {}",
            booking.id
        ));
    }
}

pub struct NotificationSystem {
    observers: Arc<RwLock<Vec<Arc<dyn BookingObserver>>>>,
}

impl NotificationSystem {
    pub fn new() -> Self {
        Self {
            observers: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn subscribe(&self, observer: Arc<dyn BookingObserver>) {
        self.observers.write().push(observer);
    }

    pub fn notify_created(&self, booking: &Booking) {
        for observer in self.observers.read().iter() {
            observer.on_booking_created(booking);
        }
    }

    pub fn notify_confirmed(&self, booking: &Booking) {
        for observer in self.observers.read().iter() {
            observer.on_booking_confirmed(booking);
        }
    }

    pub fn notify_cancelled(&self, booking: &Booking) {
        for observer in self.observers.read().iter() {
            observer.on_booking_cancelled(booking);
        }
    }

    pub fn notify_completed(&self, booking: &Booking) {
        for observer in self.observers.read().iter() {
            observer.on_booking_completed(booking);
        }
    }

    pub fn notify_status_change(&self, booking: &Booking) {
        match booking.status {
            BookingStatus::Pending => self.notify_created(booking),
            BookingStatus::Confirmed => self.notify_confirmed(booking),
            BookingStatus::Cancelled => self.notify_cancelled(booking),
            BookingStatus::Completed => self.notify_completed(booking),
        }
    }
}

impl Default for NotificationSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Local;
    use uuid::Uuid;

    #[test]
    fn test_notification_system() {
        let system = NotificationSystem::new();
        
        system.subscribe(Arc::new(EmailNotifier::new()));
        system.subscribe(Arc::new(SMSNotifier::new()));
        system.subscribe(Arc::new(AuditLogger::new()));

        let booking = Booking::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            Local::now(),
            Local::now(),
            "Test".to_string(),
            5,
        );

        system.notify_created(&booking);
        // All observers should be notified
    }
}
