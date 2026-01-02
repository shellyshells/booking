// =============================================================================
// FACTORY METHOD & ABSTRACT FACTORY PATTERNS - Room Creation
// =============================================================================
// Problem Solved: Creates room objects dynamically based on type without
//                 exposing instantiation logic. Allows adding new room types
//                 without modifying existing code (Open/Closed Principle)
// Location: Used in RoomService when creating new rooms
// =============================================================================

use super::flyweight::RoomTypeInfo;
use crate::models::room::{Room, RoomEquipment, RoomType};
use crate::patterns::singleton::log_info;
use uuid::Uuid;

// -----------------------------------------------------------------------------
// Abstract Factory Trait - Defines interface for creating room families
// -----------------------------------------------------------------------------
pub trait RoomFactory: Send + Sync {
    fn create_room(&self, name: String, capacity: u32, floor: i32) -> Room;
    fn get_room_type(&self) -> RoomType;
    fn get_default_equipment(&self) -> Vec<RoomEquipment>;
    fn get_hourly_rate(&self) -> f64;
}

// -----------------------------------------------------------------------------
// Concrete Factory: Conference Room Factory
// -----------------------------------------------------------------------------
pub struct ConferenceRoomFactory;

impl RoomFactory for ConferenceRoomFactory {
    fn create_room(&self, name: String, capacity: u32, floor: i32) -> Room {
        log_info(
            &format!("Creating conference room: {}", name),
            Some("ConferenceRoomFactory"),
        );
        Room::new(
            Uuid::new_v4().to_string(),
            name,
            RoomType::Conference,
            capacity,
            floor,
            self.get_default_equipment(),
            self.get_hourly_rate(),
        )
    }

    fn get_room_type(&self) -> RoomType {
        RoomType::Conference
    }

    fn get_default_equipment(&self) -> Vec<RoomEquipment> {
        vec![
            RoomEquipment::Projector,
            RoomEquipment::Whiteboard,
            RoomEquipment::VideoConference,
            RoomEquipment::AirConditioning,
            RoomEquipment::WiFi,
        ]
    }

    fn get_hourly_rate(&self) -> f64 {
        50.0
    }
}

// -----------------------------------------------------------------------------
// Concrete Factory: Meeting Room Factory
// -----------------------------------------------------------------------------
pub struct MeetingRoomFactory;

impl RoomFactory for MeetingRoomFactory {
    fn create_room(&self, name: String, capacity: u32, floor: i32) -> Room {
        log_info(
            &format!("Creating meeting room: {}", name),
            Some("MeetingRoomFactory"),
        );
        Room::new(
            Uuid::new_v4().to_string(),
            name,
            RoomType::Meeting,
            capacity,
            floor,
            self.get_default_equipment(),
            self.get_hourly_rate(),
        )
    }

    fn get_room_type(&self) -> RoomType {
        RoomType::Meeting
    }

    fn get_default_equipment(&self) -> Vec<RoomEquipment> {
        vec![
            RoomEquipment::Whiteboard,
            RoomEquipment::WiFi,
            RoomEquipment::AirConditioning,
        ]
    }

    fn get_hourly_rate(&self) -> f64 {
        25.0
    }
}

// -----------------------------------------------------------------------------
// Concrete Factory: Training Room Factory
// -----------------------------------------------------------------------------
pub struct TrainingRoomFactory;

impl RoomFactory for TrainingRoomFactory {
    fn create_room(&self, name: String, capacity: u32, floor: i32) -> Room {
        log_info(
            &format!("Creating training room: {}", name),
            Some("TrainingRoomFactory"),
        );
        Room::new(
            Uuid::new_v4().to_string(),
            name,
            RoomType::Training,
            capacity,
            floor,
            self.get_default_equipment(),
            self.get_hourly_rate(),
        )
    }

    fn get_room_type(&self) -> RoomType {
        RoomType::Training
    }

    fn get_default_equipment(&self) -> Vec<RoomEquipment> {
        vec![
            RoomEquipment::Projector,
            RoomEquipment::Whiteboard,
            RoomEquipment::Computer,
            RoomEquipment::AirConditioning,
            RoomEquipment::WiFi,
        ]
    }

    fn get_hourly_rate(&self) -> f64 {
        40.0
    }
}

// -----------------------------------------------------------------------------
// Concrete Factory: Auditorium Factory
// -----------------------------------------------------------------------------
pub struct AuditoriumFactory;

impl RoomFactory for AuditoriumFactory {
    fn create_room(&self, name: String, capacity: u32, floor: i32) -> Room {
        log_info(
            &format!("Creating auditorium: {}", name),
            Some("AuditoriumFactory"),
        );
        Room::new(
            Uuid::new_v4().to_string(),
            name,
            RoomType::Auditorium,
            capacity,
            floor,
            self.get_default_equipment(),
            self.get_hourly_rate(),
        )
    }

    fn get_room_type(&self) -> RoomType {
        RoomType::Auditorium
    }

    fn get_default_equipment(&self) -> Vec<RoomEquipment> {
        vec![
            RoomEquipment::Projector,
            RoomEquipment::SoundSystem,
            RoomEquipment::VideoConference,
            RoomEquipment::AirConditioning,
            RoomEquipment::WiFi,
            RoomEquipment::Microphone,
        ]
    }

    fn get_hourly_rate(&self) -> f64 {
        100.0
    }
}

// -----------------------------------------------------------------------------
// Concrete Factory: Private Office Factory
// -----------------------------------------------------------------------------
pub struct PrivateOfficeFactory;

impl RoomFactory for PrivateOfficeFactory {
    fn create_room(&self, name: String, capacity: u32, floor: i32) -> Room {
        log_info(
            &format!("Creating private office: {}", name),
            Some("PrivateOfficeFactory"),
        );
        Room::new(
            Uuid::new_v4().to_string(),
            name,
            RoomType::PrivateOffice,
            capacity,
            floor,
            self.get_default_equipment(),
            self.get_hourly_rate(),
        )
    }

    fn get_room_type(&self) -> RoomType {
        RoomType::PrivateOffice
    }

    fn get_default_equipment(&self) -> Vec<RoomEquipment> {
        vec![
            RoomEquipment::Computer,
            RoomEquipment::Phone,
            RoomEquipment::AirConditioning,
            RoomEquipment::WiFi,
        ]
    }

    fn get_hourly_rate(&self) -> f64 {
        15.0
    }
}

// -----------------------------------------------------------------------------
// Abstract Factory Manager - Provides the correct factory based on room type
// -----------------------------------------------------------------------------
pub struct RoomFactoryManager;

impl RoomFactoryManager {
    /// Returns the appropriate factory for the given room type
    pub fn get_factory(room_type: &RoomType) -> Box<dyn RoomFactory> {
        log_info(
            &format!("Getting factory for room type: {:?}", room_type),
            Some("RoomFactoryManager"),
        );
        
        match room_type {
            RoomType::Conference => Box::new(ConferenceRoomFactory),
            RoomType::Meeting => Box::new(MeetingRoomFactory),
            RoomType::Training => Box::new(TrainingRoomFactory),
            RoomType::Auditorium => Box::new(AuditoriumFactory),
            RoomType::PrivateOffice => Box::new(PrivateOfficeFactory),
        }
    }

    /// Creates a room using the appropriate factory
    pub fn create_room(
        room_type: &RoomType,
        name: String,
        capacity: u32,
        floor: i32,
    ) -> Room {
        let factory = Self::get_factory(room_type);
        factory.create_room(name, capacity, floor)
    }

    /// Creates a room with custom equipment and rate
    pub fn create_custom_room(
        room_type: &RoomType,
        name: String,
        capacity: u32,
        floor: i32,
        equipment: Vec<RoomEquipment>,
        hourly_rate: f64,
    ) -> Room {
        log_info(
            &format!("Creating custom {} room: {}", room_type.as_str(), name),
            Some("RoomFactoryManager"),
        );
        
        Room::new(
            Uuid::new_v4().to_string(),
            name,
            room_type.clone(),
            capacity,
            floor,
            equipment,
            hourly_rate,
        )
    }

    /// Get room type info using flyweight pattern
    pub fn get_room_type_info(room_type: &RoomType) -> RoomTypeInfo {
        use super::flyweight::ROOM_TYPE_FLYWEIGHT;
        ROOM_TYPE_FLYWEIGHT.get_room_info(room_type)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conference_room_factory() {
        let factory = ConferenceRoomFactory;
        let room = factory.create_room("Test Conference".to_string(), 20, 1);
        
        assert_eq!(room.room_type, RoomType::Conference);
        assert_eq!(room.capacity, 20);
        assert!(room.equipment.contains(&RoomEquipment::Projector));
    }

    #[test]
    fn test_factory_manager() {
        let room = RoomFactoryManager::create_room(
            &RoomType::Meeting,
            "Test Meeting".to_string(),
            8,
            2,
        );
        
        assert_eq!(room.room_type, RoomType::Meeting);
        assert_eq!(room.capacity, 8);
    }
}
