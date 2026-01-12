// =============================================================================
// SINGLETON PATTERN - Activity Logger & Configuration Manager
// =============================================================================
// Problem Solved: Ensures only one instance of logger/config exists globally
// Location: Used throughout the application for logging and configuration
// =============================================================================

use chrono::Local;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::{Arc, RwLock};

// -----------------------------------------------------------------------------
// Activity Log Entry - Tracks user actions
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityEntry {
    pub timestamp: String,
    pub action: String,
    pub details: String,
    pub user: Option<String>,
    pub is_error: bool,
}

// -----------------------------------------------------------------------------
// Activity Logger Singleton - Thread-safe global activity tracker
// -----------------------------------------------------------------------------
pub struct ActivityLogger {
    entries: RwLock<VecDeque<ActivityEntry>>,
    max_entries: usize,
}

impl ActivityLogger {
    fn new() -> Self {
        ActivityLogger {
            entries: RwLock::new(VecDeque::with_capacity(500)),
            max_entries: 500,
        }
    }

    /// Log a reservation creation
    pub fn log_reservation_created(&self, reservation_id: &str, user_name: &str, user_email: &str, room_name: &str, date: &str) {
        self.add_entry(ActivityEntry {
            timestamp: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            action: "RESERVATION_CREATED".to_string(),
            details: format!("Room '{}' booked for {}", room_name, date),
            user: Some(format!("{} ({})", user_name, user_email)),
            is_error: false,
        });
        println!("[ACTIVITY] Reservation created: {} by {} for {}", reservation_id, user_name, room_name);
    }

    /// Log a reservation cancellation
    pub fn log_reservation_cancelled(&self, reservation_id: &str, user_name: &str, room_name: &str) {
        self.add_entry(ActivityEntry {
            timestamp: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            action: "RESERVATION_CANCELLED".to_string(),
            details: format!("Booking for room '{}' was cancelled", room_name),
            user: Some(user_name.to_string()),
            is_error: false,
        });
        println!("[ACTIVITY] Reservation cancelled: {} by {}", reservation_id, user_name);
    }

    /// Log a reservation confirmation
    pub fn log_reservation_confirmed(&self, reservation_id: &str, user_name: &str, room_name: &str) {
        self.add_entry(ActivityEntry {
            timestamp: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            action: "RESERVATION_CONFIRMED".to_string(),
            details: format!("Booking for room '{}' was confirmed", room_name),
            user: Some(user_name.to_string()),
            is_error: false,
        });
        println!("[ACTIVITY] Reservation confirmed: {} by {}", reservation_id, user_name);
    }

    /// Log a check-in
    pub fn log_checkin(&self, reservation_id: &str, user_name: &str, room_name: &str) {
        self.add_entry(ActivityEntry {
            timestamp: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            action: "CHECK_IN".to_string(),
            details: format!("Checked in to room '{}'", room_name),
            user: Some(user_name.to_string()),
            is_error: false,
        });
        println!("[ACTIVITY] Check-in: {} by {}", reservation_id, user_name);
    }

    /// Log a room creation
    pub fn log_room_created(&self, room_name: &str, room_type: &str) {
        self.add_entry(ActivityEntry {
            timestamp: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            action: "ROOM_CREATED".to_string(),
            details: format!("New {} room '{}' added to system", room_type, room_name),
            user: Some("Admin".to_string()),
            is_error: false,
        });
        println!("[ACTIVITY] Room created: {} ({})", room_name, room_type);
    }

    /// Log an error
    pub fn log_error(&self, error_message: &str, context: Option<&str>) {
        self.add_entry(ActivityEntry {
            timestamp: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            action: "ERROR".to_string(),
            details: error_message.to_string(),
            user: context.map(|s| s.to_string()),
            is_error: true,
        });
        println!("[ERROR] {}: {}", context.unwrap_or("System"), error_message);
    }

    /// Log a validation error
    pub fn log_validation_error(&self, user_name: &str, error: &str) {
        self.add_entry(ActivityEntry {
            timestamp: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            action: "VALIDATION_ERROR".to_string(),
            details: error.to_string(),
            user: Some(user_name.to_string()),
            is_error: true,
        });
        println!("[VALIDATION ERROR] {}: {}", user_name, error);
    }

    fn add_entry(&self, entry: ActivityEntry) {
        let mut entries = self.entries.write().unwrap();
        if entries.len() >= self.max_entries {
            entries.pop_front();
        }
        entries.push_back(entry);
    }

    /// Get recent activity entries
    pub fn get_entries(&self, count: usize) -> Vec<ActivityEntry> {
        let entries = self.entries.read().unwrap();
        entries.iter().rev().take(count).cloned().collect()
    }

    /// Get only error entries
    pub fn get_errors(&self, count: usize) -> Vec<ActivityEntry> {
        let entries = self.entries.read().unwrap();
        entries.iter().rev().filter(|e| e.is_error).take(count).cloned().collect()
    }

    /// Clear all entries
    pub fn clear(&self) {
        let mut entries = self.entries.write().unwrap();
        entries.clear();
    }
}

// Global singleton instance
pub static ACTIVITY_LOG: Lazy<Arc<ActivityLogger>> = Lazy::new(|| Arc::new(ActivityLogger::new()));

// Backward-compatible log functions (no-op for internal pattern logging)
// These exist only to avoid breaking the other pattern modules
#[inline]
pub fn log_info(_message: &str, _context: Option<&str>) {}
#[inline]
pub fn log_warning(_message: &str, _context: Option<&str>) {}
#[inline] 
pub fn log_error(_message: &str, _context: Option<&str>) {}

// -----------------------------------------------------------------------------
// Configuration Singleton - Global application settings
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub server_host: String,
    pub server_port: u16,
    pub max_reservation_days: u32,
    pub min_reservation_duration_minutes: u32,
    pub max_reservation_duration_hours: u32,
    pub business_hours_start: u8,
    pub business_hours_end: u8,
    pub allow_weekend_reservations: bool,
    pub default_currency: String,
    pub timezone: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            server_host: "127.0.0.1".to_string(),
            server_port: 8080,
            max_reservation_days: 90,
            min_reservation_duration_minutes: 30,
            max_reservation_duration_hours: 8,
            business_hours_start: 8,
            business_hours_end: 22,
            allow_weekend_reservations: true,
            default_currency: "EUR".to_string(),
            timezone: "Europe/Paris".to_string(),
        }
    }
}

pub struct ConfigManager {
    config: RwLock<AppConfig>,
}

impl ConfigManager {
    fn new() -> Self {
        ConfigManager {
            config: RwLock::new(AppConfig::default()),
        }
    }

    pub fn get(&self) -> AppConfig {
        self.config.read().unwrap().clone()
    }
}

// Global configuration singleton
pub static CONFIG: Lazy<Arc<ConfigManager>> = Lazy::new(|| Arc::new(ConfigManager::new()));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_activity_logger_singleton() {
        ACTIVITY_LOG.log_reservation_created("res-1", "John", "john@test.com", "Room Alpha", "2026-01-15");
        ACTIVITY_LOG.log_reservation_cancelled("res-1", "John", "Room Alpha");
        
        let entries = ACTIVITY_LOG.get_entries(10);
        assert!(entries.len() >= 2);
    }

    #[test]
    fn test_config_singleton() {
        let config1 = CONFIG.get();
        let config2 = CONFIG.get();
        
        assert_eq!(config1.server_port, config2.server_port);
    }
}
