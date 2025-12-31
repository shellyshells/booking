use once_cell::sync::Lazy;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};

/// Global configuration using Singleton pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub max_booking_duration_hours: u32,
    pub min_booking_duration_hours: u32,
    pub max_advance_booking_days: u32,
    pub allow_concurrent_bookings: bool,
    pub default_room_capacity: u32,
    pub business_hours_start: u32,
    pub business_hours_end: u32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            max_booking_duration_hours: 8,
            min_booking_duration_hours: 1,
            max_advance_booking_days: 90,
            allow_concurrent_bookings: false,
            default_room_capacity: 10,
            business_hours_start: 8,
            business_hours_end: 20,
        }
    }
}

static CONFIG: Lazy<RwLock<Settings>> = Lazy::new(|| RwLock::new(Settings::default()));

pub struct Config;

impl Config {
    pub fn get() -> Settings {
        CONFIG.read().clone()
    }

    pub fn update<F>(f: F)
    where
        F: FnOnce(&mut Settings),
    {
        let mut config = CONFIG.write();
        f(&mut config);
    }

    pub fn set(settings: Settings) {
        *CONFIG.write() = settings;
    }
}
