use crate::models::{Room, RoomType};

/// Abstract Factory Pattern - Creates families of related rooms
pub trait AbstractRoomFactory {
    fn create_standard_room(&self, name: String, floor: u32) -> Room;
    fn create_premium_room(&self, name: String, floor: u32) -> Room;
}

pub struct StandardBuildingFactory;
pub struct PremiumBuildingFactory;

impl AbstractRoomFactory for StandardBuildingFactory {
    fn create_standard_room(&self, name: String, floor: u32) -> Room {
        Room::new(name, RoomType::MeetingRoom, 8, floor)
            .with_amenities(false, true, false)
            .with_rate(35.0)
    }

    fn create_premium_room(&self, name: String, floor: u32) -> Room {
        Room::new(name, RoomType::ConferenceRoom, 15, floor)
            .with_amenities(true, true, false)
            .with_rate(60.0)
    }
}

impl AbstractRoomFactory for PremiumBuildingFactory {
    fn create_standard_room(&self, name: String, floor: u32) -> Room {
        Room::new(name, RoomType::ConferenceRoom, 20, floor)
            .with_amenities(true, true, true)
            .with_rate(90.0)
    }

    fn create_premium_room(&self, name: String, floor: u32) -> Room {
        Room::new(name, RoomType::ExecutiveSuite, 10, floor)
            .with_amenities(true, true, true)
            .with_rate(175.0)
    }
}

pub fn get_building_factory(is_premium: bool) -> Box<dyn AbstractRoomFactory> {
    if is_premium {
        Box::new(PremiumBuildingFactory)
    } else {
        Box::new(StandardBuildingFactory)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_building() {
        let factory = StandardBuildingFactory;
        let standard = factory.create_standard_room("Room A".to_string(), 1);
        let premium = factory.create_premium_room("Room B".to_string(), 1);
        
        assert!(standard.hourly_rate < premium.hourly_rate);
        assert!(standard.capacity < premium.capacity);
    }

    #[test]
    fn test_premium_building() {
        let factory = PremiumBuildingFactory;
        let standard = factory.create_standard_room("Suite A".to_string(), 10);
        
        assert!(standard.has_video_conf);
        assert!(standard.hourly_rate > 80.0);
    }
}
