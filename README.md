# 🏢 Room Reservation System

A comprehensive room reservation system demonstrating advanced **Object-Oriented Programming (OOP) design patterns** in **Rust**. This project features a modern web interface and implements multiple Gang of Four (GoF) design patterns.

![Rust](https://img.shields.io/badge/Rust-1.75+-orange?style=flat-square&logo=rust)
![License](https://img.shields.io/badge/License-MIT-blue?style=flat-square)
![Patterns](https://img.shields.io/badge/Design_Patterns-7-green?style=flat-square)

## 🌟 Features

### Core Functionality
- **Room Management**: Create, view, update, and delete rooms of various types
- **Reservation System**: Book rooms with full validation and conflict detection
- **Real-time Dashboard**: View statistics, recent activity, and room availability
- **Filtering & Search**: Filter rooms by type, capacity, floor; filter reservations by status and date
- **User Role-based Validation**: Different validation rules for Standard, VIP, and Admin users

### Design Patterns Implemented

| Pattern | Type | Purpose |
|---------|------|---------|
| **Singleton** | Creational | Global Logger and Configuration Manager |
| **Factory Method** | Creational | Room creation based on type |
| **Abstract Factory** | Creational | Room factory management |
| **Composite** | Structural | Room groups (floors, buildings) |
| **Flyweight** | Structural | Shared room type information |
| **Strategy** | Behavioral | Validation strategies |
| **Observer** | Behavioral | Event notification system |
| **Adapter** | Structural | External calendar integration |

### Extra Features (Beyond Requirements)
- 🎨 **Beautiful Dark-themed UI** with modern aesthetics
- 📊 **Real-time Analytics Dashboard** with statistics
- 📝 **Live System Logs** viewer
- 🔔 **Toast Notifications** for user feedback
- 🎯 **Multiple Validation Strategies** showcase
- 👁️ **Active Observers** display
- 📅 **Calendar Integration Adapters** (Google, Outlook, iCal)
- 🔍 **Smart Room Search** with multiple filters
- ✅ **Check-in/Check-out System**

## 🚀 Quick Start

### Prerequisites

- **Rust** (1.75 or later): [Install Rust](https://rustup.rs/)
- **Cargo** (comes with Rust)

### Installation & Running

1. **Extract the project** (if downloaded as zip):
   ```bash
   unzip room-reservation-system.zip
   cd room-reservation-system
   ```

2. **Build the project**:
   ```bash
   cargo build --release
   ```

3. **Run the application**:
   ```bash
   cargo run --release
   ```

4. **Open your browser** and navigate to:
   ```
   http://127.0.0.1:8080
   ```

### Alternative: Development Mode

For development with hot-reloading logs:
```bash
RUST_LOG=info cargo run
```

## 📁 Project Structure

```
room-reservation-system/
├── Cargo.toml                 # Project dependencies
├── README.md                  # This file
├── src/
│   ├── main.rs               # Application entry point
│   ├── api/
│   │   └── mod.rs            # REST API handlers
│   ├── models/
│   │   ├── mod.rs            # Models module
│   │   ├── room.rs           # Room data structures
│   │   └── reservation.rs    # Reservation data structures
│   ├── patterns/
│   │   ├── mod.rs            # Patterns module
│   │   ├── singleton.rs      # Singleton: Logger & Config
│   │   ├── factory.rs        # Factory: Room creation
│   │   ├── composite.rs      # Composite: Room groups
│   │   ├── flyweight.rs      # Flyweight: Shared type info
│   │   ├── strategy.rs       # Strategy: Validation
│   │   ├── observer.rs       # Observer: Events
│   │   └── adapter.rs        # Adapter: Calendar APIs
│   ├── services/
│   │   └── mod.rs            # Business logic & state
│   └── utils/                # Utility functions
└── static/
    ├── index.html            # Main HTML page
    ├── css/
    │   └── styles.css        # Application styles
    └── js/
        └── app.js            # Frontend JavaScript
```

## 🧩 Design Patterns Explained

### 1. Singleton Pattern (`patterns/singleton.rs`)

**Problem Solved**: Ensures only one instance of Logger and Configuration exists globally.

**Implementation**:
- `LOGGER`: Thread-safe global logging system using `once_cell::Lazy`
- `CONFIG`: Application configuration manager

**Usage**:
```rust
use crate::patterns::singleton::{log_info, CONFIG};

log_info("Server starting", Some("Main"));
let config = CONFIG.get();
```

### 2. Factory Method Pattern (`patterns/factory.rs`)

**Problem Solved**: Creates room objects dynamically based on type without exposing instantiation logic.

**Implementation**:
- `RoomFactory` trait defines the interface
- Concrete factories: `ConferenceRoomFactory`, `MeetingRoomFactory`, etc.
- `RoomFactoryManager` provides the appropriate factory

**Usage**:
```rust
let room = RoomFactoryManager::create_room(
    &RoomType::Conference,
    "Room Alpha".to_string(),
    20,
    1,
);
```

### 3. Composite Pattern (`patterns/composite.rs`)

**Problem Solved**: Treats individual rooms and groups uniformly for bulk operations.

**Implementation**:
- `Reservable` trait for common interface
- `RoomGroup` can contain rooms or other groups
- Recursive capacity calculation

**Usage**:
```rust
let mut floor = RoomGroup::new("Floor 1", "First floor", RoomGroupType::Floor);
floor.add(Box::new(room1));
floor.add(Box::new(room2));
println!("Total capacity: {}", floor.get_total_capacity());
```

### 4. Flyweight Pattern (`patterns/flyweight.rs`)

**Problem Solved**: Shares immutable room type information across many room instances.

**Implementation**:
- `RoomTypeInfo` contains shared data (description, icon, default equipment)
- `ROOM_TYPE_FLYWEIGHT` caches all type information

**Benefits**:
- Memory savings when there are many rooms
- Consistent type information

### 5. Strategy Pattern (`patterns/strategy.rs`)

**Problem Solved**: Allows different validation rules for different user types.

**Strategies**:
- `StandardValidationStrategy`: All business rules enforced
- `VipValidationStrategy`: Extended limits (+20% capacity, 12h duration)
- `AdminValidationStrategy`: Minimal restrictions
- `QuietHoursValidationStrategy`: Special after-hours rules

**Usage**:
```rust
let strategy = get_strategy_for_role("vip");
let context = ValidationContext::new(strategy);
let result = context.validate(&reservation, &room, &existing);
```

### 6. Observer Pattern (`patterns/observer.rs`)

**Problem Solved**: Decouples event sources from handlers for extensible notifications.

**Observers**:
- `EmailNotifier`: Sends email notifications (simulated)
- `AnalyticsTracker`: Tracks events for analytics
- `AuditLogger`: Creates audit trail
- `SlackNotifier`: Sends Slack messages (simulated)

**Usage**:
```rust
let publisher = EventPublisher::new();
publisher.subscribe(Arc::new(EmailNotifier::new()));
publisher.publish(ReservationEvent::Created(event_data));
```

### 7. Adapter Pattern (`patterns/adapter.rs`)

**Problem Solved**: Integrates with external calendar APIs without changing core application.

**Adapters**:
- `GoogleCalendarAdapter`: Converts to Google Calendar API format
- `OutlookCalendarAdapter`: Converts to Microsoft Graph API format
- `ICalAdapter`: Generates .ics files

**Usage**:
```rust
let adapter = GoogleCalendarAdapter::new("api-key", "calendar-id");
adapter.create_event(&calendar_event);
```

## 🌐 API Endpoints

### Rooms
- `GET /api/rooms` - List all rooms
- `GET /api/rooms/{id}` - Get room details
- `POST /api/rooms` - Create a new room
- `PUT /api/rooms/{id}` - Update a room
- `DELETE /api/rooms/{id}` - Delete a room
- `GET /api/rooms/search` - Search rooms with filters

### Reservations
- `GET /api/reservations` - List reservations
- `POST /api/reservations` - Create reservation
- `GET /api/reservations/{id}` - Get reservation details
- `POST /api/reservations/{id}/cancel` - Cancel reservation
- `POST /api/reservations/{id}/confirm` - Confirm reservation
- `POST /api/reservations/{id}/checkin` - Check in

### System
- `GET /api/health` - Health check
- `GET /api/statistics` - System statistics
- `GET /api/logs` - System logs
- `GET /api/room-types` - Room type information
- `GET /api/validation-strategies` - Available strategies
- `GET /api/observers` - Active observers

## 🎨 User Interface

The web interface features:
- **Dashboard**: Statistics overview with charts
- **Rooms**: Room cards with filtering
- **Reservations**: Table view with status management
- **Book Room**: Reservation form with room preview
- **Design Patterns**: Interactive pattern documentation
- **System Logs**: Real-time log viewer

## 🧪 Running Tests

```bash
cargo test
```

## 📦 Building for Production

```bash
cargo build --release
```

The binary will be at `target/release/room-reservation-system`.

## 🛠️ Configuration

The application uses default configuration which can be modified through the `CONFIG` singleton:

```rust
CONFIG.update(|config| {
    config.server_port = 3000;
    config.max_reservation_days = 60;
});
```

## 📝 License

MIT License - Feel free to use and modify.

---

**Built with ❤️ using Rust and modern web technologies**
