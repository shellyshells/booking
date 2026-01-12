// =============================================================================
// SERVICES MODULE - Business Logic Layer
// =============================================================================
// Contains the main application state and business logic services
// =============================================================================

use crate::models::{
    CreateReservationRequest, CreateRoomRequest, Reservation, ReservationFilter,
    ReservationStatus, Room, RoomType, UpdateReservationRequest, UpdateRoomRequest,
};
use crate::patterns::{
    adapter::CalendarService,
    composite::{RoomGroupManager, RoomGroupType},
    factory::RoomFactoryManager,
    flyweight::{get_all_room_types, FlyweightStats, RoomTypeInfo, ROOM_TYPE_FLYWEIGHT},
    observer::{
        create_default_observers, EventPublisher, ObserverInfo, ReservationEvent,
        ReservationEventData,
    },
    singleton::{ActivityEntry, AppConfig, CONFIG, ACTIVITY_LOG},
    strategy::{
        get_available_strategies, get_strategy_for_role, StrategyInfo, ValidationContext,
        ValidationResult,
    },
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::RwLock;

// -----------------------------------------------------------------------------
// Application State - Central state management
// -----------------------------------------------------------------------------
pub struct AppState {
    pub rooms: RwLock<HashMap<String, Room>>,
    pub reservations: RwLock<HashMap<String, Reservation>>,
    pub room_groups: RwLock<RoomGroupManager>,
    pub event_publisher: EventPublisher,
    pub calendar_service: RwLock<CalendarService>,
}

impl AppState {
    pub fn new() -> Self {
        let state = AppState {
            rooms: RwLock::new(HashMap::new()),
            reservations: RwLock::new(HashMap::new()),
            room_groups: RwLock::new(RoomGroupManager::new()),
            event_publisher: EventPublisher::new(),
            calendar_service: RwLock::new(CalendarService::new()),
        };

        // Subscribe default observers
        for observer in create_default_observers() {
            state.event_publisher.subscribe(observer);
        }

        // Add sample data
        state.add_sample_data();

        state
    }

    fn add_sample_data(&self) {
        // Create sample rooms using Factory pattern
        let rooms_to_create = vec![
            ("Alpha", RoomType::Conference, 20, 1),
            ("Beta", RoomType::Conference, 30, 1),
            ("Gamma", RoomType::Meeting, 8, 1),
            ("Delta", RoomType::Meeting, 6, 2),
            ("Epsilon", RoomType::Meeting, 10, 2),
            ("Theta", RoomType::Training, 25, 2),
            ("Lambda", RoomType::Training, 15, 3),
            ("Omega", RoomType::Auditorium, 100, 0),
            ("Sigma", RoomType::PrivateOffice, 2, 3),
            ("Phi", RoomType::PrivateOffice, 4, 3),
        ];

        let mut rooms = self.rooms.write().unwrap();
        for (name, room_type, capacity, floor) in rooms_to_create {
            let room = RoomFactoryManager::create_room(
                &room_type,
                format!("Room {}", name),
                capacity,
                floor,
            );
            rooms.insert(room.id.clone(), room);
        }

        // Create sample room groups using Composite pattern
        let mut groups = self.room_groups.write().unwrap();
        
        // Floor 1 group
        let _floor1_id = groups.create_group(
            "Floor 1 - Executive".to_string(),
            "All rooms on the first floor - executive area".to_string(),
            RoomGroupType::Floor,
        );

        // Floor 2 group
        let _floor2_id = groups.create_group(
            "Floor 2 - Operations".to_string(),
            "Operations and training center".to_string(),
            RoomGroupType::Floor,
        );
    }

    // -------------------------------------------------------------------------
    // Room Management
    // -------------------------------------------------------------------------
    
    pub fn create_room(&self, request: CreateRoomRequest) -> Result<Room, String> {
        let room_type = RoomType::from_str(&request.room_type)
            .ok_or_else(|| format!("Invalid room type: {}", request.room_type))?;

        let room = RoomFactoryManager::create_room(
            &room_type,
            request.name,
            request.capacity,
            request.floor,
        );

        let id = room.id.clone();
        let mut rooms = self.rooms.write().unwrap();
        rooms.insert(id.clone(), room.clone());

        // Log the activity
        ACTIVITY_LOG.log_room_created(&room.name, room.room_type.as_str());
        
        Ok(room)
    }

    pub fn get_room(&self, id: &str) -> Option<Room> {
        self.rooms.read().unwrap().get(id).cloned()
    }

    pub fn list_rooms(&self) -> Vec<Room> {
        self.rooms.read().unwrap().values().cloned().collect()
    }

    pub fn update_room(&self, id: &str, request: UpdateRoomRequest) -> Result<Room, String> {
        let mut rooms = self.rooms.write().unwrap();
        let room = rooms.get_mut(id).ok_or("Room not found")?;

        if let Some(name) = request.name {
            room.name = name;
        }
        if let Some(capacity) = request.capacity {
            room.capacity = capacity;
        }
        if let Some(floor) = request.floor {
            room.floor = floor;
        }
        if let Some(rate) = request.hourly_rate {
            room.hourly_rate = rate;
        }
        if let Some(available) = request.is_available {
            room.is_available = available;
        }
        if let Some(desc) = request.description {
            room.description = Some(desc);
        }

        Ok(room.clone())
    }

    pub fn delete_room(&self, id: &str) -> Result<(), String> {
        // Check for active reservations
        let has_reservations = self.reservations.read().unwrap()
            .values()
            .any(|r| r.room_id == id && r.is_upcoming());

        if has_reservations {
            return Err("Cannot delete room with active reservations".to_string());
        }

        let mut rooms = self.rooms.write().unwrap();
        rooms.remove(id).ok_or("Room not found")?;

        Ok(())
    }

    pub fn search_rooms(&self, query: &RoomSearchQuery) -> Vec<Room> {
        let rooms = self.rooms.read().unwrap();
        rooms.values()
            .filter(|room| {
                if let Some(ref room_type) = query.room_type {
                    if room.room_type.as_str().to_lowercase() != room_type.to_lowercase() {
                        return false;
                    }
                }
                if let Some(min_cap) = query.min_capacity {
                    if room.capacity < min_cap {
                        return false;
                    }
                }
                if let Some(max_cap) = query.max_capacity {
                    if room.capacity > max_cap {
                        return false;
                    }
                }
                if let Some(floor) = query.floor {
                    if room.floor != floor {
                        return false;
                    }
                }
                if query.available_only.unwrap_or(false) && !room.is_available {
                    return false;
                }
                true
            })
            .cloned()
            .collect()
    }

    // -------------------------------------------------------------------------
    // Reservation Management (with Strategy pattern for validation)
    // -------------------------------------------------------------------------

    pub fn create_reservation(&self, request: CreateReservationRequest) -> Result<ReservationResult, String> {
        // Get the room
        let room = self.get_room(&request.room_id)
            .ok_or("Room not found")?;

        // Create reservation object
        let reservation = Reservation::new(
            request.room_id.clone(),
            request.user_name.clone(),
            request.user_email.clone(),
            request.start_time,
            request.end_time,
            request.attendees,
            request.purpose,
        );

        // Get validation strategy based on user role (Strategy Pattern)
        let role = request.user_role.as_deref().unwrap_or("standard");
        let strategy = get_strategy_for_role(role);
        let context = ValidationContext::new(strategy);

        // Get existing reservations for the room
        let existing: Vec<Reservation> = self.reservations.read().unwrap()
            .values()
            .filter(|r| r.room_id == request.room_id)
            .cloned()
            .collect();

        // Validate using the strategy
        let validation = context.validate(&reservation, &room, &existing);

        if !validation.is_valid {
            // Log validation error
            ACTIVITY_LOG.log_validation_error(
                &request.user_name,
                &validation.errors.join(", ")
            );
            return Ok(ReservationResult {
                success: false,
                reservation: None,
                validation,
            });
        }

        // Store the reservation
        let id = reservation.id.clone();
        self.reservations.write().unwrap().insert(id.clone(), reservation.clone());

        // Publish event (Observer Pattern)
        let event_data = ReservationEventData {
            reservation_id: reservation.id.clone(),
            room_id: room.id.clone(),
            room_name: room.name.clone(),
            user_email: reservation.user_email.clone(),
            user_name: reservation.user_name.clone(),
            start_time: reservation.start_time,
            end_time: reservation.end_time,
            event_time: Utc::now(),
        };
        self.event_publisher.publish(ReservationEvent::Created(event_data));

        // Log the activity
        let date_str = reservation.start_time.format("%Y-%m-%d %H:%M").to_string();
        ACTIVITY_LOG.log_reservation_created(
            &id,
            &reservation.user_name,
            &reservation.user_email,
            &room.name,
            &date_str
        );

        Ok(ReservationResult {
            success: true,
            reservation: Some(reservation),
            validation,
        })
    }

    pub fn get_reservation(&self, id: &str) -> Option<Reservation> {
        self.reservations.read().unwrap().get(id).cloned()
    }

    pub fn list_reservations(&self, filter: Option<ReservationFilter>) -> Vec<Reservation> {
        let reservations = self.reservations.read().unwrap();
        let filter = filter.unwrap_or_default();

        reservations.values()
            .filter(|r| {
                if let Some(ref room_id) = filter.room_id {
                    if &r.room_id != room_id {
                        return false;
                    }
                }
                if let Some(ref email) = filter.user_email {
                    if &r.user_email != email {
                        return false;
                    }
                }
                if let Some(ref status) = filter.status {
                    if r.status.as_str().to_lowercase() != status.to_lowercase() {
                        return false;
                    }
                }
                if let Some(from) = filter.from_date {
                    if r.start_time < from {
                        return false;
                    }
                }
                if let Some(to) = filter.to_date {
                    if r.end_time > to {
                        return false;
                    }
                }
                true
            })
            .cloned()
            .collect()
    }

    pub fn cancel_reservation(&self, id: &str) -> Result<Reservation, String> {
        let mut reservations = self.reservations.write().unwrap();
        let reservation = reservations.get_mut(id).ok_or("Reservation not found")?;

        if reservation.status == ReservationStatus::Cancelled {
            return Err("Reservation is already cancelled".to_string());
        }

        let user_name = reservation.user_name.clone();
        let room_id = reservation.room_id.clone();
        reservation.cancel();

        // Get room name and log
        let room_name = self.get_room(&room_id)
            .map(|r| r.name.clone())
            .unwrap_or_else(|| "Unknown Room".to_string());

        // Publish cancellation event
        if let Some(room) = self.get_room(&room_id) {
            let event_data = ReservationEventData {
                reservation_id: reservation.id.clone(),
                room_id: room.id.clone(),
                room_name: room.name.clone(),
                user_email: reservation.user_email.clone(),
                user_name: reservation.user_name.clone(),
                start_time: reservation.start_time,
                end_time: reservation.end_time,
                event_time: Utc::now(),
            };
            drop(reservations); // Release lock before publishing
            self.event_publisher.publish(ReservationEvent::Cancelled(event_data));
        } else {
            drop(reservations);
        }

        // Log the cancellation
        ACTIVITY_LOG.log_reservation_cancelled(id, &user_name, &room_name);
        
        self.get_reservation(id).ok_or("Reservation not found".to_string())
    }

    pub fn confirm_reservation(&self, id: &str) -> Result<Reservation, String> {
        let mut reservations = self.reservations.write().unwrap();
        let reservation = reservations.get_mut(id).ok_or("Reservation not found")?;
        
        let user_name = reservation.user_name.clone();
        let room_id = reservation.room_id.clone();
        reservation.confirm();
        let result = reservation.clone();
        drop(reservations);

        // Get room name and log
        let room_name = self.get_room(&room_id)
            .map(|r| r.name.clone())
            .unwrap_or_else(|| "Unknown Room".to_string());
        
        ACTIVITY_LOG.log_reservation_confirmed(id, &user_name, &room_name);
        
        Ok(result)
    }

    pub fn check_in(&self, id: &str) -> Result<Reservation, String> {
        let mut reservations = self.reservations.write().unwrap();
        let reservation = reservations.get_mut(id).ok_or("Reservation not found")?;
        
        let user_name = reservation.user_name.clone();
        let room_id = reservation.room_id.clone();
        reservation.check_in();
        let result = reservation.clone();
        drop(reservations);

        // Get room name and log
        let room_name = self.get_room(&room_id)
            .map(|r| r.name.clone())
            .unwrap_or_else(|| "Unknown Room".to_string());
        
        ACTIVITY_LOG.log_checkin(id, &user_name, &room_name);
        
        Ok(result)
    }

    // -------------------------------------------------------------------------
    // Room Availability
    // -------------------------------------------------------------------------

    pub fn get_room_availability(&self, room_id: &str, date: DateTime<Utc>) -> RoomAvailability {
        let reservations = self.reservations.read().unwrap();
        let day_start = date.date_naive().and_hms_opt(0, 0, 0).unwrap();
        let day_end = date.date_naive().and_hms_opt(23, 59, 59).unwrap();

        let day_reservations: Vec<TimeSlot> = reservations.values()
            .filter(|r| {
                r.room_id == room_id
                    && r.status != ReservationStatus::Cancelled
                    && r.start_time.date_naive() == date.date_naive()
            })
            .map(|r| TimeSlot {
                start: r.start_time,
                end: r.end_time,
                reservation_id: Some(r.id.clone()),
            })
            .collect();

        RoomAvailability {
            room_id: room_id.to_string(),
            date,
            booked_slots: day_reservations,
        }
    }

    // -------------------------------------------------------------------------
    // Statistics & Analytics
    // -------------------------------------------------------------------------

    pub fn get_statistics(&self) -> SystemStatistics {
        let rooms = self.rooms.read().unwrap();
        let reservations = self.reservations.read().unwrap();
        let now = Utc::now();

        let total_rooms = rooms.len();
        let available_rooms = rooms.values().filter(|r| r.is_available).count();
        
        let total_reservations = reservations.len();
        let active_reservations = reservations.values()
            .filter(|r| r.is_active())
            .count();
        let upcoming_reservations = reservations.values()
            .filter(|r| r.is_upcoming())
            .count();
        let cancelled_reservations = reservations.values()
            .filter(|r| r.status == ReservationStatus::Cancelled)
            .count();

        // Room utilization for today
        let today_reservations: Vec<_> = reservations.values()
            .filter(|r| {
                r.start_time.date_naive() == now.date_naive()
                    && r.status != ReservationStatus::Cancelled
            })
            .collect();

        let total_booked_hours: f64 = today_reservations.iter()
            .map(|r| r.duration_hours())
            .sum();

        let room_stats_by_type: HashMap<String, u32> = rooms.values()
            .fold(HashMap::new(), |mut acc, room| {
                *acc.entry(room.room_type.as_str().to_string()).or_insert(0) += 1;
                acc
            });

        SystemStatistics {
            total_rooms,
            available_rooms,
            total_reservations,
            active_reservations,
            upcoming_reservations,
            cancelled_reservations,
            today_bookings: today_reservations.len(),
            today_total_hours: total_booked_hours,
            rooms_by_type: room_stats_by_type,
            flyweight_stats: ROOM_TYPE_FLYWEIGHT.memory_stats(total_rooms),
        }
    }

    // -------------------------------------------------------------------------
    // Configuration & System
    // -------------------------------------------------------------------------

    pub fn get_config(&self) -> AppConfig {
        CONFIG.get()
    }

    pub fn get_logs(&self, count: usize) -> Vec<ActivityEntry> {
        ACTIVITY_LOG.get_entries(count)
    }

    pub fn get_room_types(&self) -> Vec<RoomTypeInfo> {
        get_all_room_types()
    }

    pub fn get_validation_strategies(&self) -> Vec<StrategyInfo> {
        get_available_strategies()
    }

    pub fn get_observers(&self) -> Vec<ObserverInfo> {
        self.event_publisher.list_observers()
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

// -----------------------------------------------------------------------------
// Supporting Types
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomSearchQuery {
    pub room_type: Option<String>,
    pub min_capacity: Option<u32>,
    pub max_capacity: Option<u32>,
    pub floor: Option<i32>,
    pub available_only: Option<bool>,
    pub has_equipment: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReservationResult {
    pub success: bool,
    pub reservation: Option<Reservation>,
    pub validation: ValidationResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSlot {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub reservation_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomAvailability {
    pub room_id: String,
    pub date: DateTime<Utc>,
    pub booked_slots: Vec<TimeSlot>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStatistics {
    pub total_rooms: usize,
    pub available_rooms: usize,
    pub total_reservations: usize,
    pub active_reservations: usize,
    pub upcoming_reservations: usize,
    pub cancelled_reservations: usize,
    pub today_bookings: usize,
    pub today_total_hours: f64,
    pub rooms_by_type: HashMap<String, u32>,
    pub flyweight_stats: FlyweightStats,
}
