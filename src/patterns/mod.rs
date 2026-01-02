// =============================================================================
// DESIGN PATTERNS MODULE
// =============================================================================
// This module contains implementations of various design patterns used
// throughout the Room Reservation System.
// =============================================================================

pub mod singleton;   // Singleton Pattern - Logger & Configuration
pub mod factory;     // Factory Pattern - Room Creation
pub mod composite;   // Composite Pattern - Room Groups
pub mod flyweight;   // Flyweight Pattern - Shared Room Type Info
pub mod strategy;    // Strategy Pattern - Validation Strategies
pub mod observer;    // Observer Pattern - Event Notifications
pub mod adapter;     // Adapter Pattern - External Calendar Integration

// Re-export commonly used items for convenience
pub use singleton::{log_info, log_error, log_warning, log_debug, CONFIG, LOGGER};
pub use factory::{RoomFactory, RoomFactoryManager};
pub use composite::{Reservable, ReservableComponent, RoomGroup, RoomGroupType, RoomGroupManager};
pub use flyweight::{get_room_type_info, get_all_room_types, ROOM_TYPE_FLYWEIGHT};
pub use strategy::{ValidationStrategy, ValidationContext, ValidationResult, get_strategy_for_role};
pub use observer::{EventPublisher, ReservationEvent, ReservationEventData, create_default_observers};
pub use adapter::{CalendarAdapter, CalendarEvent, CalendarService};
