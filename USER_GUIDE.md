# Room Booking System - User Guide

## Getting Started

### Installation

1. Ensure you have Rust installed (version 1.70 or higher)
2. Clone or extract the project
3. Navigate to the project directory
4. Build the project:
   ```bash
   cargo build --release
   ```

### Running the Application

```bash
cargo run --release
```

The application will launch with a graphical interface.

## User Interface Overview

### Main Dashboard

The dashboard provides an overview of the system:
- **Total Rooms**: Number of available rooms
- **Active Bookings**: Current and upcoming reservations
- **Total Users**: Registered system users
- **Recent Bookings**: List of the latest bookings

### Navigation

Use the left sidebar to navigate between sections:
- 📊 **Dashboard**: System overview
- 📅 **Bookings**: Manage reservations
- 🏢 **Rooms**: View and create rooms
- 👥 **Users**: User management
- ⚙️ **Settings**: System configuration

## Features

### Managing Bookings

#### View Bookings
1. Click on "📅 Bookings" in the sidebar
2. See all bookings in a table format with:
   - Room name
   - Purpose
   - Time
   - Status
   - Actions

#### Confirm a Booking
1. Find a pending booking
2. Click the "✓ Confirm" button
3. The booking status changes to "Confirmed"
4. Notifications are sent automatically

#### Cancel a Booking
1. Find an active booking
2. Click the "✗ Cancel" button
3. The booking is marked as "Cancelled"

### Managing Rooms

#### View Rooms
1. Click on "🏢 Rooms" in the sidebar
2. See all rooms with:
   - Name and type
   - Capacity
   - Floor number
   - Hourly rate
   - Available amenities (📽️ Projector, 📋 Whiteboard, 📹 Video Conference)

#### Add a New Room
1. Go to the Rooms section
2. Click on "➕ Add New Room" to expand the form
3. Fill in:
   - **Name**: Room identifier
   - **Type**: Select from dropdown (Meeting Room, Conference Room, etc.)
   - **Capacity**: Maximum number of people
   - **Floor**: Floor number
4. Click "Create Room"

Room types available:
- **Meeting Room**: Small teams, 8 people, $40/hour
- **Conference Room**: Large meetings, 20 people, $75/hour
- **Training Room**: Workshops, 25 people, $60/hour
- **Executive Suite**: Executive meetings, 6 people, $150/hour
- **Auditorium**: Large events, 100 people, $200/hour

### User Management

View all registered users with their:
- Name and email
- Department
- Role (Employee, Manager, Administrator)

Different roles have different permissions:
- **Employee**: Can book rooms
- **Manager**: Can manage bookings and override some restrictions
- **Administrator**: Full system access and configuration

### System Settings

Configure global system parameters:

- **Max Booking Duration**: Maximum hours for a single booking (1-24)
- **Min Booking Duration**: Minimum hours for a booking (1-8)
- **Business Hours Start**: When bookings can begin (0-23)
- **Business Hours End**: When bookings must end (0-23)
- **Allow Concurrent Bookings**: Enable/disable overlapping bookings

Changes take effect immediately and apply to all new bookings.

## Advanced Features

### Room Groups (Composite Pattern)

Programmatically manage multiple rooms as a unit:

```rust
use room_booking_system::composite::{RoomGroup, RoomComponent};

let mut group = RoomGroup::new("Conference Suite".to_string());
group.add(room1)?;
group.add(room2)?;

// Book all rooms in the group
let bookings = group.book_all(user_id, start_time, end_time);
```

### Validation Strategies

The system uses different validation strategies:

- **Standard**: Enforces all business rules
  - No past bookings
  - Within business hours
  - Respects duration limits
  - Checks capacity
  - Prevents double-booking

- **Priority** (Managers): More flexible
  - Allows capacity overflow
  - Permits some conflicts (requires approval)

- **Flexible** (Testing): Minimal restrictions

### Pricing Strategies

Different pricing models can be applied:

- **Standard**: Base hourly rate
- **Peak Hours**: 1.5x rate during 9-11am and 2-4pm
- **Discount**: 10% off for 4+ hours, 20% off for 8+ hours
- **Weekend**: 1.25x rate for weekends

### Calendar Integration

Bookings can be synced to external calendars:

```rust
use room_booking_system::adapter::{CalendarAdapter, GoogleCalendarAPI};

let adapter = CalendarAdapter::new(GoogleCalendarAPI);
adapter.sync_booking(&booking, room_name)?;
```

Supported systems:
- Google Calendar
- Microsoft Outlook

### Notifications

Automatic notifications are sent for:
- ✉️ **Email**: All booking events
- 📱 **SMS**: Creation, confirmation, cancellation
- 📋 **Audit Log**: Complete audit trail

## Troubleshooting

### Common Issues

**Problem**: Cannot create booking
- **Solution**: Check that:
  - Booking is not in the past
  - Time is within business hours
  - Duration is between min and max limits
  - Room capacity is sufficient
  - No conflicting bookings exist

**Problem**: Room creation fails
- **Solution**: Ensure:
  - Capacity is a valid number
  - Floor number is valid
  - Room name is not empty

**Problem**: Settings changes don't apply
- **Solution**: 
  - Click "Save Settings" button
  - Restart the application if issues persist

### Logs

All system actions are logged to `booking_system.log` in the application directory. Check this file for detailed error messages.

## Keyboard Shortcuts

- **Tab**: Navigate between fields
- **Enter**: Submit forms (when applicable)
- **Escape**: Close dialogs

## Tips & Best Practices

1. **Regular Cleanup**: Periodically review and cancel unused bookings
2. **Capacity Planning**: Monitor room usage patterns
3. **Advance Booking**: Book popular rooms early
4. **Business Hours**: Respect configured business hours for best results
5. **Room Types**: Choose appropriate room type for your needs

## Support

For issues or questions:
1. Check the logs in `booking_system.log`
2. Review this user guide
3. Consult the technical documentation in `ARCHITECTURE.md`

## Version Information

- **Version**: 1.0.0
- **Rust Edition**: 2021
- **GUI Framework**: egui 0.28

---

*This guide covers the main features of the Room Booking System. For technical details about design patterns and architecture, see ARCHITECTURE.md.*
