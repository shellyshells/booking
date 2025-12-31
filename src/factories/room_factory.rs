use crate::models::{Room, RoomType};

/// Factory Method Pattern - Creates different types of rooms
pub trait RoomFactory {
    fn create_room(&self, name: String, floor: u32) -> Room;
}

pub struct ConferenceRoomFactory;
pub struct MeetingRoomFactory;
pub struct TrainingRoomFactory;
pub struct ExecutiveSuiteFactory;
pub struct AuditoriumFactory;

impl RoomFactory for ConferenceRoomFactory {
    fn create_room(&self, name: String, floor: u32) -> Room {
        Room::new(name, RoomType::ConferenceRoom, 20, floor)
            .with_amenities(true, true, true)
            .with_rate(75.0)
    }
}

impl RoomFactory for MeetingRoomFactory {
    fn create_room(&self, name: String, floor: u32) -> Room {
        Room::new(name, RoomType::MeetingRoom, 8, floor)
            .with_amenities(false, true, false)
            .with_rate(40.0)
    }
}

impl RoomFactory for TrainingRoomFactory {
    fn create_room(&self, name: String, floor: u32) -> Room {
        Room::new(name, RoomType::TrainingRoom, 25, floor)
            .with_amenities(true, true, false)
            .with_rate(60.0)
    }
}

impl RoomFactory for ExecutiveSuiteFactory {
    fn create_room(&self, name: String, floor: u32) -> Room {
        Room::new(name, RoomType::ExecutiveSuite, 6, floor)
            .with_amenities(true, true, true)
            .with_rate(150.0)
    }
}

impl RoomFactory for AuditoriumFactory {
    fn create_room(&self, name: String, floor: u32) -> Room {
        Room::new(name, RoomType::Auditorium, 100, floor)
            .with_amenities(true, false, true)
            .with_rate(200.0)
    }
}

/// Simple Factory Pattern - Central factory for all room types
pub struct SimpleRoomFactory;

impl SimpleRoomFactory {
    pub fn create_room(room_type: RoomType, name: String, floor: u32) -> Room {
        match room_type {
            RoomType::ConferenceRoom => ConferenceRoomFactory.create_room(name, floor),
            RoomType::MeetingRoom => MeetingRoomFactory.create_room(name, floor),
            RoomType::TrainingRoom => TrainingRoomFactory.create_room(name, floor),
            RoomType::ExecutiveSuite => ExecutiveSuiteFactory.create_room(name, floor),
            RoomType::Auditorium => AuditoriumFactory.create_room(name, floor),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conference_room_factory() {
        let room = ConferenceRoomFactory.create_room("Board Room".to_string(), 5);
        assert_eq!(room.room_type, RoomType::ConferenceRoom);
        assert_eq!(room.capacity, 20);
        assert!(room.has_projector);
        assert!(room.has_video_conf);
    }

    #[test]
    fn test_simple_factory() {
        let room = SimpleRoomFactory::create_room(
            RoomType::MeetingRoom,
            "Quick Meet".to_string(),
            2,
        );
        assert_eq!(room.room_type, RoomType::MeetingRoom);
        assert_eq!(room.capacity, 8);
    }
}
