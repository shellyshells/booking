// =============================================================================
// SINGLETON PATTERN - Logger & Configuration Manager
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
// Log Entry Structure
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: LogLevel,
    pub message: String,
    pub context: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warning => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERROR"),
        }
    }
}

// -----------------------------------------------------------------------------
// Logger Singleton - Thread-safe global logger
// -----------------------------------------------------------------------------
pub struct Logger {
    logs: RwLock<VecDeque<LogEntry>>,
    max_entries: usize,
    min_level: RwLock<LogLevel>,
}

impl Logger {
    fn new() -> Self {
        Logger {
            logs: RwLock::new(VecDeque::with_capacity(1000)),
            max_entries: 1000,
            min_level: RwLock::new(LogLevel::Debug),
        }
    }

    // Log a message with a given level
    pub fn log(&self, level: LogLevel, message: &str, context: Option<&str>) {
        let min_level = self.min_level.read().unwrap();
        if !self.should_log(&level, &min_level) {
            return;
        }
        drop(min_level);

        let entry = LogEntry {
            timestamp: Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string(),
            level: level.clone(),
            message: message.to_string(),
            context: context.map(|s| s.to_string()),
        };

        // Print to console
        let ctx = context.unwrap_or("-");
        println!("[{}] [{}] [{}] {}", entry.timestamp, level, ctx, message);

        // Store in memory buffer
        let mut logs = self.logs.write().unwrap();
        if logs.len() >= self.max_entries {
            logs.pop_front();
        }
        logs.push_back(entry);
    }

    fn should_log(&self, level: &LogLevel, min_level: &LogLevel) -> bool {
        let level_priority = |l: &LogLevel| match l {
            LogLevel::Debug => 0,
            LogLevel::Info => 1,
            LogLevel::Warning => 2,
            LogLevel::Error => 3,
        };
        level_priority(level) >= level_priority(min_level)
    }

    // Convenience methods for different log levels
    pub fn debug(&self, message: &str, context: Option<&str>) {
        self.log(LogLevel::Debug, message, context);
    }

    pub fn info(&self, message: &str, context: Option<&str>) {
        self.log(LogLevel::Info, message, context);
    }

    pub fn warning(&self, message: &str, context: Option<&str>) {
        self.log(LogLevel::Warning, message, context);
    }

    pub fn error(&self, message: &str, context: Option<&str>) {
        self.log(LogLevel::Error, message, context);
    }

    // Retrieve recent logs
    pub fn get_logs(&self, count: usize) -> Vec<LogEntry> {
        let logs = self.logs.read().unwrap();
        logs.iter().rev().take(count).cloned().collect()
    }

    // Set minimum log level
    pub fn set_min_level(&self, level: LogLevel) {
        let mut min_level = self.min_level.write().unwrap();
        *min_level = level;
    }

    // Clear all logs
    pub fn clear(&self) {
        let mut logs = self.logs.write().unwrap();
        logs.clear();
    }
}

// Global singleton instance using Lazy initialization
pub static LOGGER: Lazy<Arc<Logger>> = Lazy::new(|| Arc::new(Logger::new()));

// Convenient global functions
pub fn log_debug(message: &str, context: Option<&str>) {
    LOGGER.debug(message, context);
}

pub fn log_info(message: &str, context: Option<&str>) {
    LOGGER.info(message, context);
}

pub fn log_warning(message: &str, context: Option<&str>) {
    LOGGER.warning(message, context);
}

pub fn log_error(message: &str, context: Option<&str>) {
    LOGGER.error(message, context);
}

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

    pub fn update<F>(&self, updater: F)
    where
        F: FnOnce(&mut AppConfig),
    {
        let mut config = self.config.write().unwrap();
        updater(&mut config);
        log_info("Configuration updated", Some("ConfigManager"));
    }
}

// Global configuration singleton
pub static CONFIG: Lazy<Arc<ConfigManager>> = Lazy::new(|| Arc::new(ConfigManager::new()));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logger_singleton() {
        // Both calls should return the same instance
        log_info("Test message 1", Some("Test"));
        log_info("Test message 2", Some("Test"));
        
        let logs = LOGGER.get_logs(10);
        assert!(logs.len() >= 2);
    }

    #[test]
    fn test_config_singleton() {
        let config1 = CONFIG.get();
        let config2 = CONFIG.get();
        
        assert_eq!(config1.server_port, config2.server_port);
    }
}
