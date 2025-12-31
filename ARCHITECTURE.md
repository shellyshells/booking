# Architecture Documentation

## Design Patterns Analysis

This document provides detailed analysis of each design pattern implementation, explaining the problems they solve and their application in the room booking system.

---

## 1. Singleton Pattern

### Implementations
1. **Logger** (`src/logger/singleton_logger.rs`)
2. **Configuration** (`src/config/settings.rs`)
3. **Metadata Factory** (`src/flyweight/room_metadata.rs`)

### Problem Addressed
In a room booking system, certain components must be globally accessible and exist as a single instance:
- **Logger**: Multiple loggers would create conflicting log files and consume unnecessary resources
- **Configuration**: System settings must be consistent across all components
- **Metadata Factory**: Centralized management of shared room type information

### Implementation Details

**Logger Singleton**:
```rust
static LOGGER: Lazy<Logger> = Lazy::new(|| {
    Logger {
        log_file: Mutex::new(None),
    }
});
```
- Uses `Lazy` from `once_cell` for thread-safe initialization
- `Mutex` ensures thread-safe file writing
- Single instance accessed via `Logger::instance()`

**Configuration Singleton**:
```rust
static CONFIG: Lazy<RwLock<Settings>> = Lazy::new(|| 
    RwLock::new(Settings::default())
);
```
- `RwLock` allows multiple readers, single writer
- Thread-safe access to global settings
- Runtime updates without restarting

### Benefits
- **Memory Efficiency**: Only one instance in memory
- **Consistency**: All parts of the system use the same configuration
- **Thread Safety**: Concurrent access is properly synchronized
- **Global Access**: Easy access from any part of the codebase

---

## 2. Factory Method Pattern

### Implementation
**Location**: `src/factories/room_factory.rs`

### Problem Addressed
Creating room objects with different configurations based on type becomes complex and error-prone when done manually. Each room type has specific:
- Default capacity
- Standard amenities
- Base pricing
- Recommended equipment

### Solution
Define a `RoomFactory` trait and implement it for each room type:

```rust
pub trait RoomFactory {
    fn create_room(&self, name: String, floor: u32) -> Room;
}

impl RoomFactory for ConferenceRoomFactory {
    fn create_room(&self, name: String, floor: u32) -> Room {
        Room::new(name, RoomType::ConferenceRoom, 20, floor)
            .with_amenities(true, true, true)
            .with_rate(75.0)
    }
}
```

### Benefits
- **Encapsulation**: Room creation logic is encapsulated
- **Consistency**: All conference rooms have the same default configuration
- **Extensibility**: New room types can be added easily
- **Type Safety**: Compile-time guarantees about room configuration

### Use Cases
- Creating rooms in the UI
- Initializing the system with default rooms
- Testing with consistent room configurations

---

## 3. Abstract Factory Pattern

### Implementation
**Location**: `src/factories/abstract_factory.rs`

### Problem Addressed
The system needs to create families of related rooms. A standard office building has different room specifications than a premium building. Without this pattern, we would need to manually track and apply building-specific configurations.

### Solution
```rust
pub trait AbstractRoomFactory {
    fn create_standard_room(&self, name: String, floor: u32) -> Room;
    fn create_premium_room(&self, name: String, floor: u32) -> Room;
}

impl AbstractRoomFactory for PremiumBuildingFactory {
    fn create_standard_room(&self, name: String, floor: u32) -> Room {
        Room::new(name, RoomType::ConferenceRoom, 20, floor)
            .with_amenities(true, true, true)
            .with_rate(90.0)
    }
    // ...
}
```

### Benefits
- **Consistency**: All rooms in a building follow the same quality standard
- **Flexibility**: Easy to add new building types
- **Coordination**: Related rooms are created together with consistent properties

### Use Cases
- Multi-building organizations
- Different office locations with different standards
- Testing different building configurations

---

## 4. Composite Pattern

### Implementation
**Location**: `src/composite/room_group.rs`

### Problem Addressed
Sometimes bookings require multiple rooms (large events, training sessions). Without Composite:
- Each room must be booked individually
- Total capacity must be calculated manually
- Managing groups is complex and error-prone

### Solution
```rust
pub trait RoomComponent {
    fn add(&mut self, room: Room) -> Result<(), String>;
    fn remove(&mut self, id: Uuid) -> Result<(), String>;
    fn total_capacity(&self) -> u32;
}

pub struct RoomGroup {
    rooms: Vec<Room>,
}
```

### Benefits
- **Unified Interface**: Work with single rooms or groups uniformly
- **Aggregate Operations**: Book all rooms in a group at once
- **Capacity Management**: Automatically calculate total capacity
- **Flexibility**: Groups can be composed and decomposed dynamically

### Use Cases
- Conference suites (multiple connected rooms)
- Large events requiring several rooms
- Department-specific room allocations

---

## 5. Flyweight Pattern

### Implementation
**Location**: `src/flyweight/room_metadata.rs`

### Problem Addressed
Each room type has immutable characteristics (description, typical capacity, recommended amenities). Storing this data in every room instance wastes memory, especially with hundreds of rooms.

### Solution
```rust
pub struct RoomMetadata {
    pub room_type: RoomType,
    pub description: String,
    pub typical_capacity: u32,
    pub base_hourly_rate: f64,
    pub recommended_amenities: Vec<String>,
}

static METADATA_FACTORY: Lazy<Mutex<MetadataFactory>> = Lazy::new(|| {
    Mutex::new(MetadataFactory::new())
});
```

### Memory Savings
**Without Flyweight** (1000 rooms, 5 types):
- Each room stores ~200 bytes of metadata
- Total: 1000 × 200 = 200KB

**With Flyweight**:
- 5 shared instances × 200 bytes = 1KB
- Rooms store only a reference (8 bytes)
- Total: 1KB + (1000 × 8) = 9KB
- **Saving: 95%**

### Benefits
- **Memory Efficiency**: Share immutable data across instances
- **Performance**: Faster object creation
- **Consistency**: Guaranteed consistent metadata per type

---

## 6. Strategy Pattern

### Implementations
1. **Validation Strategy** (`src/strategy/validation_strategy.rs`)
2. **Pricing Strategy** (`src/strategy/pricing_strategy.rs`)

### Problem Addressed

**Validation**: Different users or contexts require different booking rules:
- Standard users: strict business hour enforcement
- Managers: flexible capacity limits
- Executives: priority booking with minimal restrictions

**Pricing**: Different pricing models based on business needs:
- Standard hourly rates
- Peak hour premiums
- Volume discounts
- Weekend pricing

### Solution
```rust
pub trait ValidationStrategy {
    fn validate(&self, booking: &Booking, room: &Room, 
                user: &User, existing: &[Booking]) -> Result<(), String>;
}

pub trait PricingStrategy {
    fn calculate_price(&self, base_rate: f64, duration: f64, 
                      booking: &Booking) -> f64;
}
```

### Benefits
- **Flexibility**: Switch strategies at runtime
- **Extensibility**: Add new strategies without modifying existing code
- **Testability**: Each strategy can be tested independently
- **Maintainability**: Changes to one strategy don't affect others

### Use Cases
- **Validation**: Different rules for different user types
- **Pricing**: Promotional periods, special events, off-peak discounts

---

## 7. Adapter Pattern

### Implementation
**Location**: `src/adapter/calendar_adapter.rs`

### Problem Addressed
The system needs to integrate with external calendar systems (Google Calendar, Outlook) that have different APIs and data formats. Without Adapter, the booking system would need specific code for each calendar type.

### Solution
```rust
pub trait ExternalCalendarSystem {
    fn create_event(&self, event: ExternalCalendarEvent) 
        -> Result<String, String>;
    fn update_event(&self, event: ExternalCalendarEvent) 
        -> Result<(), String>;
    fn delete_event(&self, event_id: &str) 
        -> Result<(), String>;
}

pub struct CalendarAdapter<T: ExternalCalendarSystem> {
    calendar: T,
}
```

### Benefits
- **Decoupling**: Booking system independent of calendar APIs
- **Flexibility**: Easy to add support for new calendar systems
- **Consistency**: Unified interface for all calendar operations
- **Testability**: Can mock calendar systems for testing

### Use Cases
- Syncing bookings to Google Calendar
- Integrating with Outlook
- Adding support for custom calendar systems

---

## 8. Observer Pattern

### Implementation
**Location**: `src/observer/notification_system.rs`

### Problem Addressed
When a booking is created, confirmed, or cancelled, multiple actions must occur:
- Send email notification
- Send SMS alert
- Log to audit trail
- Update dashboards

Without Observer, the booking logic would be tightly coupled to all notification mechanisms.

### Solution
```rust
pub trait BookingObserver {
    fn on_booking_created(&self, booking: &Booking);
    fn on_booking_confirmed(&self, booking: &Booking);
    fn on_booking_cancelled(&self, booking: &Booking);
}

pub struct NotificationSystem {
    observers: Vec<Arc<dyn BookingObserver>>,
}
```

### Benefits
- **Decoupling**: Booking logic independent of notification mechanisms
- **Extensibility**: Add new observers without modifying booking code
- **Flexibility**: Enable/disable notifications dynamically
- **Parallel Processing**: Observers can be notified concurrently

### Use Cases
- Email notifications
- SMS alerts
- Audit logging
- Real-time dashboard updates
- Analytics tracking

---

## System Integration

### Pattern Interactions

```
User Action (Book Room)
    ↓
[Factory] Creates Room
    ↓
[Strategy] Validates Booking
    ↓
[Strategy] Calculates Price
    ↓
Booking Created
    ↓
[Observer] Notifies All Subscribers
    ├→ [Adapter] Syncs to External Calendar
    ├→ Email Notification
    ├→ SMS Notification
    └→ [Singleton] Logs to Audit Trail
```

### Data Flow

1. **Room Creation**: Factory patterns ensure consistent room setup
2. **Booking Validation**: Strategy pattern applies appropriate rules
3. **Price Calculation**: Strategy pattern determines cost
4. **Event Notification**: Observer pattern broadcasts changes
5. **External Sync**: Adapter pattern integrates with calendars
6. **Logging**: Singleton logger records all actions

### Thread Safety

- **Singleton instances**: Protected by `Mutex` or `RwLock`
- **Flyweight cache**: Thread-safe access to shared metadata
- **Observer notifications**: Observers must implement `Send + Sync`
- **UI state**: Protected by `Arc<RwLock<T>>` for concurrent access

---

## Performance Considerations

### Memory Optimization
- **Flyweight**: Reduces memory for room metadata by 95%
- **Arc**: Shared ownership without copying large structures
- **Lazy initialization**: Singletons created only when needed

### Runtime Efficiency
- **Strategy selection**: O(1) strategy switching
- **Observer notification**: Parallel notifications possible
- **Composite operations**: Bulk operations on room groups

### Scalability
- **Factory pattern**: Fast room creation without complex logic
- **Adapter pattern**: Multiple calendar integrations without code duplication
- **Observer pattern**: Add unlimited observers without performance impact

---

## Testing Strategy

Each pattern has dedicated unit tests:

1. **Singleton**: Verify single instance and thread safety
2. **Factory**: Test correct object creation for all types
3. **Abstract Factory**: Verify family consistency
4. **Composite**: Test aggregate operations
5. **Flyweight**: Verify memory sharing
6. **Strategy**: Test all strategy variants
7. **Adapter**: Mock external systems
8. **Observer**: Verify all observers are notified

Run tests:
```bash
cargo test
```

---

## Conclusion

The design patterns implemented in this system provide:
- **Maintainability**: Clear separation of concerns
- **Extensibility**: Easy to add new features
- **Performance**: Optimized memory and runtime efficiency
- **Reliability**: Thread-safe concurrent operations
- **Testability**: Each pattern can be tested independently

This architecture demonstrates best practices in object-oriented design applied to a real-world booking system.
