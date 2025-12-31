# Room Booking System

A sophisticated room booking management system built in Rust, demonstrating advanced object-oriented design patterns with a modern graphical interface.

## Overview

This system enables organizations to manage meeting room bookings efficiently while showcasing best practices in software architecture through the implementation of multiple design patterns.

## Features

### Core Functionality
- **Room Management**: Create and manage different types of rooms (Conference, Meeting, Training, Executive Suite, Auditorium)
- **Booking System**: Schedule, confirm, cancel, and track room reservations
- **User Management**: Handle users with different roles and permissions
- **Validation**: Configurable booking validation with business rules
- **Pricing**: Flexible pricing strategies (standard, peak hours, discounts, weekend rates)
- **Notifications**: Multi-channel notification system (Email, SMS, Audit logs)
- **Calendar Integration**: Adapter for external calendar systems (Google Calendar, Outlook)

### Advanced Features
- **Room Groups**: Composite pattern for managing multiple rooms as a single unit
- **Smart Pricing**: Dynamic pricing based on time, duration, and demand
- **Capacity Management**: Intelligent room allocation based on attendee count
- **Audit Trail**: Complete logging of all system actions
- **Configuration Management**: Global settings with runtime updates

### Graphical Interface
- Modern, responsive UI built with egui
- Dashboard with real-time statistics
- Intuitive booking management
- Room catalog with detailed information
- User administration panel
- Settings configuration

## Design Patterns Implemented

### 1. Singleton Pattern
**Purpose**: Ensure only one instance of critical system components

**Implementation**:
- `Logger`: Global logging system
- `Config`: Application configuration

**Problem Solved**: Prevents multiple instances of system-wide resources, ensuring consistency and reducing memory overhead.

**Location**: 
- `src/logger/singleton_logger.rs`
- `src/config/settings.rs`

### 2. Factory Method Pattern
**Purpose**: Create objects without specifying their exact classes

**Implementation**:
- `RoomFactory` trait with specific factories for each room type
- `ConferenceRoomFactory`, `MeetingRoomFactory`, etc.

**Problem Solved**: Encapsulates room creation logic, making it easy to add new room types without modifying existing code.

**Location**: `src/factories/room_factory.rs`

### 3. Abstract Factory Pattern
**Purpose**: Create families of related objects

**Implementation**:
- `AbstractRoomFactory` for creating standard vs. premium rooms
- `StandardBuildingFactory` and `PremiumBuildingFactory`

**Problem Solved**: Allows creating different "families" of rooms (standard building vs. premium building) with consistent characteristics.

**Location**: `src/factories/abstract_factory.rs`

### 4. Composite Pattern
**Purpose**: Treat individual objects and compositions uniformly

**Implementation**:
- `RoomGroup` that can contain multiple rooms
- `RoomComponent` trait for unified interface

**Problem Solved**: Enables booking multiple rooms together, calculating total capacity, and managing room collections as single entities.

**Location**: `src/composite/room_group.rs`

### 5. Flyweight Pattern
**Purpose**: Share common data to reduce memory usage

**Implementation**:
- `RoomMetadata` shared across rooms of the same type
- `MetadataFactory` for managing shared instances

**Problem Solved**: Reduces memory usage by sharing immutable room type information (descriptions, typical capacity, amenities) across all rooms of the same type.

**Location**: `src/flyweight/room_metadata.rs`

### 6. Strategy Pattern
**Purpose**: Define a family of algorithms and make them interchangeable

**Implementation**:
- **Validation Strategies**: `StandardValidation`, `PriorityValidation`, `FlexibleValidation`
- **Pricing Strategies**: `StandardPricing`, `PeakHoursPricing`, `DiscountPricing`, `WeekendPricing`

**Problem Solved**: 
- Allows different booking validation rules based on user type or context
- Enables flexible pricing models that can be changed at runtime

**Location**: `src/strategy/`

### 7. Adapter Pattern
**Purpose**: Convert one interface into another

**Implementation**:
- `CalendarAdapter` for integrating external calendar systems
- Support for Google Calendar and Outlook

**Problem Solved**: Enables integration with different external calendar APIs without changing our booking system's core logic.

**Location**: `src/adapter/calendar_adapter.rs`

### 8. Observer Pattern
**Purpose**: Notify multiple objects about state changes

**Implementation**:
- `NotificationSystem` as the subject
- `EmailNotifier`, `SMSNotifier`, `AuditLogger` as observers

**Problem Solved**: Decouples booking events from notification mechanisms, allowing easy addition of new notification channels.

**Location**: `src/observer/notification_system.rs`

## Architecture

```
┌─────────────────────────────────────────┐
│         Graphical Interface (egui)      │
├─────────────────────────────────────────┤
│           Application Logic             │
│  ┌──────────────┐  ┌──────────────┐    │
│  │  Factories   │  │   Strategy   │    │
│  ├──────────────┤  ├──────────────┤    │
│  │  Composite   │  │   Adapter    │    │
│  ├──────────────┤  ├──────────────┤    │
│  │  Flyweight   │  │   Observer   │    │
│  └──────────────┘  └──────────────┘    │
├─────────────────────────────────────────┤
│          Domain Models                  │
│  (Room, Booking, User)                  │
├─────────────────────────────────────────┤
│   Infrastructure (Logger, Config)       │
└─────────────────────────────────────────┘
```

## Installation

### Prerequisites
- Rust 1.70 or higher
- Cargo

### Build
```bash
cd room-booking-system
cargo build --release
```

### Run
```bash
cargo run --release
```

### Test
```bash
cargo test
```

## Usage

### Creating a Room
```rust
use room_booking_system::factories::SimpleRoomFactory;
use room_booking_system::models::RoomType;

let room = SimpleRoomFactory::create_room(
    RoomType::ConferenceRoom,
    "Board Room".to_string(),
    5 // floor number
);
```

### Making a Booking
```rust
use room_booking_system::models::Booking;
use chrono::{Local, Duration};

let booking = Booking::new(
    room_id,
    user_id,
    Local::now() + Duration::hours(1),
    Local::now() + Duration::hours(3),
    "Team Meeting".to_string(),
    8, // attendees
);
```

### Using Validation Strategy
```rust
use room_booking_system::strategy::{BookingValidator, StandardValidation};

let validator = BookingValidator::new(Box::new(StandardValidation));
let result = validator.validate(&booking, &room, &user, &existing_bookings);
```

### Managing Room Groups
```rust
use room_booking_system::composite::{RoomGroup, RoomComponent};

let mut group = RoomGroup::new("Conference Suite".to_string());
group.add(room1)?;
group.add(room2)?;

let total_capacity = group.total_capacity();
```

## Configuration

Settings can be modified in the UI or programmatically:

```rust
use room_booking_system::config::Config;

Config::update(|settings| {
    settings.max_booking_duration_hours = 8;
    settings.business_hours_start = 8;
    settings.business_hours_end = 18;
});
```

## Testing

The project includes comprehensive unit tests for all design patterns:

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific pattern tests
cargo test test_singleton
cargo test test_factory
cargo test test_composite
```

## Project Structure

```
room-booking-system/
├── src/
│   ├── main.rs              # Application entry point
│   ├── lib.rs               # Library root
│   ├── config/              # Singleton configuration
│   ├── logger/              # Singleton logger
│   ├── models/              # Domain models
│   ├── factories/           # Factory patterns
│   ├── composite/           # Composite pattern
│   ├── flyweight/           # Flyweight pattern
│   ├── strategy/            # Strategy patterns
│   ├── adapter/             # Adapter pattern
│   ├── observer/            # Observer pattern
│   ├── ui/                  # Graphical interface
│   └── tests/               # Test suite
├── Cargo.toml
├── README.md
└── ARCHITECTURE.md
```

## Technology Stack

- **Language**: Rust 2021 Edition
- **GUI Framework**: egui/eframe
- **Date/Time**: chrono
- **Serialization**: serde
- **Unique IDs**: uuid
- **Concurrency**: parking_lot, once_cell

## License

This project is developed for educational purposes demonstrating advanced OOP design patterns in Rust.

## Contributors

Developed as part of an advanced object-oriented programming course.
