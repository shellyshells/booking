# Design Patterns Summary

## Quick Reference Guide

This document provides a concise overview of all design patterns implemented in the Room Booking System.

---

## Pattern Index

| Pattern | Type | Problem Solved | Location |
|---------|------|----------------|----------|
| Singleton | Creational | Global instance management | `logger/`, `config/`, `flyweight/` |
| Factory Method | Creational | Object creation flexibility | `factories/room_factory.rs` |
| Abstract Factory | Creational | Family of related objects | `factories/abstract_factory.rs` |
| Composite | Structural | Tree structure operations | `composite/room_group.rs` |
| Flyweight | Structural | Memory optimization | `flyweight/room_metadata.rs` |
| Adapter | Structural | Interface compatibility | `adapter/calendar_adapter.rs` |
| Strategy | Behavioral | Algorithm selection | `strategy/` |
| Observer | Behavioral | Event notification | `observer/notification_system.rs` |

---

## 1. SINGLETON PATTERN

### Purpose
Ensure a class has only one instance and provide global access to it.

### Implementations

#### Logger
```rust
Logger::instance().info("Application started");
```
- **Problem**: Multiple log files, resource conflicts
- **Solution**: Single global logger instance

#### Configuration
```rust
let settings = Config::get();
Config::update(|s| s.max_booking_duration_hours = 10);
```
- **Problem**: Inconsistent settings across components
- **Solution**: Single source of truth for configuration

### When to Use
- Need exactly one instance of a class
- Global access point required
- Resource sharing (file handles, connections)

---

## 2. FACTORY METHOD PATTERN

### Purpose
Define an interface for creating objects, but let subclasses decide which class to instantiate.

### Implementation
```rust
pub trait RoomFactory {
    fn create_room(&self, name: String, floor: u32) -> Room;
}

// Use it
let factory = ConferenceRoomFactory;
let room = factory.create_room("Board Room".to_string(), 5);
```

### Factories Available
- `ConferenceRoomFactory` - 20 capacity, full amenities, $75/hr
- `MeetingRoomFactory` - 8 capacity, whiteboard, $40/hr
- `TrainingRoomFactory` - 25 capacity, projector + whiteboard, $60/hr
- `ExecutiveSuiteFactory` - 6 capacity, premium amenities, $150/hr
- `AuditoriumFactory` - 100 capacity, A/V equipment, $200/hr

### When to Use
- Creation logic is complex
- Want to delegate instantiation to subclasses
- Need consistent object initialization

---

## 3. ABSTRACT FACTORY PATTERN

### Purpose
Provide an interface for creating families of related objects.

### Implementation
```rust
pub trait AbstractRoomFactory {
    fn create_standard_room(&self, name: String, floor: u32) -> Room;
    fn create_premium_room(&self, name: String, floor: u32) -> Room;
}

// Use it
let factory = PremiumBuildingFactory;
let room = factory.create_standard_room("Suite A".to_string(), 10);
```

### Factory Families
- **StandardBuildingFactory**: Economy rooms with basic amenities
- **PremiumBuildingFactory**: Luxury rooms with premium amenities

### When to Use
- Need to create families of related products
- Products must be used together
- Want to enforce product compatibility

---

## 4. COMPOSITE PATTERN

### Purpose
Compose objects into tree structures to represent part-whole hierarchies.

### Implementation
```rust
let mut group = RoomGroup::new("Conference Suite".to_string());
group.add(room1)?;
group.add(room2)?;

let total_capacity = group.total_capacity();
let bookings = group.book_all(user_id, start, end);
```

### Features
- Treat individual rooms and groups uniformly
- Aggregate operations (book all, total capacity)
- Dynamic composition

### When to Use
- Represent hierarchies (tree structures)
- Want clients to treat individual and composite objects uniformly
- Need aggregate operations

---

## 5. FLYWEIGHT PATTERN

### Purpose
Use sharing to support large numbers of fine-grained objects efficiently.

### Implementation
```rust
let metadata = MetadataFactory::get_metadata(RoomType::ConferenceRoom);

// All conference rooms share this same metadata instance
println!("Description: {}", metadata.description);
println!("Base rate: ${}", metadata.base_hourly_rate);
```

### Shared Data (Intrinsic State)
- Room type description
- Typical capacity
- Base hourly rate
- Recommended amenities

### Unique Data (Extrinsic State)
- Room name
- Room ID
- Floor number
- Current availability

### Memory Savings
For 1000 rooms across 5 types:
- **Without Flyweight**: ~200KB
- **With Flyweight**: ~9KB
- **Savings**: 95%

### When to Use
- Many objects share common data
- Storage costs are high
- Object identity doesn't matter

---

## 6. ADAPTER PATTERN

### Purpose
Convert the interface of a class into another interface clients expect.

### Implementation
```rust
let adapter = CalendarAdapter::new(GoogleCalendarAPI);
let event_id = adapter.sync_booking(&booking, "Room A")?;

// Or use Outlook
let adapter = CalendarAdapter::new(OutlookCalendarAPI);
adapter.sync_booking(&booking, "Room A")?;
```

### Adapts
- **Google Calendar API** → Internal booking format
- **Outlook Calendar API** → Internal booking format

### When to Use
- Want to use existing class with incompatible interface
- Need to integrate third-party libraries
- Want to reuse legacy code

---

## 7. STRATEGY PATTERN

### Purpose
Define a family of algorithms, encapsulate each one, and make them interchangeable.

### Implementations

#### Validation Strategy
```rust
// Standard validation
let validator = BookingValidator::new(Box::new(StandardValidation));

// Or priority validation for managers
let validator = BookingValidator::new(Box::new(PriorityValidation));

// Or flexible validation for testing
let validator = BookingValidator::new(Box::new(FlexibleValidation));

validator.validate(&booking, &room, &user, &existing_bookings)?;
```

**Validation Strategies**:
- `StandardValidation`: Strict business rules
- `PriorityValidation`: Flexible for managers
- `FlexibleValidation`: Minimal restrictions

#### Pricing Strategy
```rust
// Standard pricing
let calculator = PricingCalculator::new(Box::new(StandardPricing));

// Or peak hours pricing
let calculator = PricingCalculator::new(Box::new(PeakHoursPricing));

// Or discount pricing
let calculator = PricingCalculator::new(Box::new(DiscountPricing));

let price = calculator.calculate(50.0, &booking);
```

**Pricing Strategies**:
- `StandardPricing`: Base rate × hours
- `PeakHoursPricing`: 1.5× during peak times
- `DiscountPricing`: 10-20% off for longer bookings
- `WeekendPricing`: 1.25× on weekends

### When to Use
- Multiple variants of an algorithm
- Need to switch algorithms at runtime
- Want to isolate algorithm implementation

---

## 8. OBSERVER PATTERN

### Purpose
Define a one-to-many dependency so when one object changes state, all dependents are notified.

### Implementation
```rust
let system = NotificationSystem::new();

// Subscribe observers
system.subscribe(Arc::new(EmailNotifier::new()));
system.subscribe(Arc::new(SMSNotifier::new()));
system.subscribe(Arc::new(AuditLogger::new()));

// Notify all observers
system.notify_created(&booking);
system.notify_confirmed(&booking);
system.notify_cancelled(&booking);
```

### Observers
- **EmailNotifier**: Sends email notifications
- **SMSNotifier**: Sends SMS alerts
- **AuditLogger**: Records to audit log

### Notification Events
- `on_booking_created`
- `on_booking_confirmed`
- `on_booking_cancelled`
- `on_booking_completed`

### When to Use
- One object's state change affects many others
- Don't know how many objects need notification
- Want loose coupling between objects

---

## Pattern Combinations

### Example: Creating and Booking a Room

```rust
// 1. FACTORY: Create room
let room = SimpleRoomFactory::create_room(
    RoomType::ConferenceRoom,
    "Board Room".to_string(),
    5
);

// 2. STRATEGY: Validate booking
let validator = BookingValidator::new(Box::new(StandardValidation));
validator.validate(&booking, &room, &user, &[])?;

// 3. STRATEGY: Calculate price
let calculator = PricingCalculator::new(Box::new(DiscountPricing));
let price = calculator.calculate(room.hourly_rate, &booking);

// 4. OBSERVER: Notify stakeholders
notification_system.notify_created(&booking);

// 5. ADAPTER: Sync to external calendar
let adapter = CalendarAdapter::new(GoogleCalendarAPI);
adapter.sync_booking(&booking, &room.name)?;

// 6. SINGLETON: Log the action
Logger::instance().info("Booking created successfully");
```

---

## Testing Patterns

Each pattern has dedicated unit tests:

```bash
# Test all patterns
cargo test

# Test specific patterns
cargo test test_singleton
cargo test test_factory
cargo test test_composite
cargo test test_flyweight
cargo test test_strategy
cargo test test_adapter
cargo test test_observer
```

---

## Benefits Summary

| Pattern | Main Benefit |
|---------|-------------|
| Singleton | Resource management & consistency |
| Factory Method | Flexible object creation |
| Abstract Factory | Product family consistency |
| Composite | Unified tree operations |
| Flyweight | Memory efficiency |
| Adapter | Interface compatibility |
| Strategy | Algorithm flexibility |
| Observer | Loose coupling & notifications |

---

## Anti-Patterns to Avoid

### ❌ DON'T: Singleton Overuse
```rust
// Wrong: Making everything a singleton
static DATABASE: Lazy<Database> = ...;
static CACHE: Lazy<Cache> = ...;
static VALIDATOR: Lazy<Validator> = ...;
```

### ✅ DO: Use Dependency Injection
```rust
// Better: Inject dependencies
struct BookingService {
    database: Arc<Database>,
    cache: Arc<Cache>,
    validator: Box<dyn Validator>,
}
```

### ❌ DON'T: God Object Factory
```rust
// Wrong: One factory for everything
impl SuperFactory {
    fn create_room(...) -> Room { }
    fn create_user(...) -> User { }
    fn create_booking(...) -> Booking { }
    // ...
}
```

### ✅ DO: Focused Factories
```rust
// Better: Separate factories
impl RoomFactory { ... }
impl UserFactory { ... }
impl BookingFactory { ... }
```

---

## Further Reading

- **Book**: "Design Patterns: Elements of Reusable Object-Oriented Software" (Gang of Four)
- **Rust Patterns**: https://rust-unofficial.github.io/patterns/
- **Architecture**: See `ARCHITECTURE.md` for detailed analysis
- **Usage**: See `USER_GUIDE.md` for practical examples

---

*This summary provides quick reference for all design patterns in the Room Booking System. For detailed implementation and rationale, consult the ARCHITECTURE.md document.*
