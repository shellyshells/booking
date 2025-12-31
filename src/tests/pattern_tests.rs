use room_booking_system::*;
use chrono::{Local, Duration};

#[cfg(test)]
mod pattern_tests {
    use super::*;

    #[test]
    fn test_singleton_logger() {
        let logger1 = logger::Logger::instance();
        let logger2 = logger::Logger::instance();
        assert!(std::ptr::eq(logger1, logger2), "Logger should be a singleton");
    }

    #[test]
    fn test_singleton_config() {
        use config::Config;
        
        let initial = Config::get();
        Config::update(|settings| {
            settings.max_booking_duration_hours = 10;
        });
        
        let updated = Config::get();
        assert_eq!(updated.max_booking_duration_hours, 10);
    }

    #[test]
    fn test_factory_method() {
        use factories::{RoomFactory, ConferenceRoomFactory};
        use models::RoomType;
        
        let factory = ConferenceRoomFactory;
        let room = factory.create_room("Test Room".to_string(), 1);
        
        assert_eq!(room.room_type, RoomType::ConferenceRoom);
        assert_eq!(room.capacity, 20);
        assert!(room.has_projector);
    }

    #[test]
    fn test_simple_factory() {
        use factories::SimpleRoomFactory;
        use models::RoomType;
        
        let room = SimpleRoomFactory::create_room(
            RoomType::MeetingRoom,
            "Quick Room".to_string(),
            2
        );
        
        assert_eq!(room.room_type, RoomType::MeetingRoom);
    }

    #[test]
    fn test_abstract_factory() {
        use factories::{AbstractRoomFactory, PremiumBuildingFactory};
        
        let factory = PremiumBuildingFactory;
        let standard = factory.create_standard_room("Room A".to_string(), 1);
        let premium = factory.create_premium_room("Room B".to_string(), 1);
        
        assert!(premium.hourly_rate > standard.hourly_rate);
        assert!(premium.has_video_conf);
    }

    #[test]
    fn test_composite_pattern() {
        use composite::{RoomGroup, RoomComponent};
        use models::{Room, RoomType};
        
        let mut group = RoomGroup::new("Test Suite".to_string());
        
        let room1 = Room::new("R1".to_string(), RoomType::MeetingRoom, 10, 1);
        let room2 = Room::new("R2".to_string(), RoomType::MeetingRoom, 15, 1);
        
        assert!(group.add(room1).is_ok());
        assert!(group.add(room2).is_ok());
        
        assert_eq!(group.total_capacity(), 25);
        assert_eq!(group.get_rooms().len(), 2);
    }

    #[test]
    fn test_flyweight_pattern() {
        use flyweight::MetadataFactory;
        use models::RoomType;
        
        let meta1 = MetadataFactory::get_metadata(RoomType::ConferenceRoom);
        let meta2 = MetadataFactory::get_metadata(RoomType::ConferenceRoom);
        
        // Verify same Arc instance
        assert!(std::sync::Arc::ptr_eq(&meta1, &meta2));
        assert_eq!(meta1.typical_capacity, 20);
    }

    #[test]
    fn test_validation_strategy() {
        use strategy::{ValidationStrategy, StandardValidation};
        use models::{Room, RoomType, User, Booking};
        use uuid::Uuid;
        
        let validator = StandardValidation;
        let room = Room::new("Test".to_string(), RoomType::MeetingRoom, 10, 1);
        let user = User::new("Test".to_string(), "test@test.com".to_string(), "IT".to_string());
        
        let booking = Booking::new(
            room.id,
            user.id,
            Local::now() + Duration::hours(1),
            Local::now() + Duration::hours(3),
            "Test Meeting".to_string(),
            5,
        );

        let result = validator.validate(&booking, &room, &user, &[]);
        assert!(result.is_ok(), "Valid booking should pass validation");
    }

    #[test]
    fn test_pricing_strategy() {
        use strategy::{PricingStrategy, StandardPricing, DiscountPricing};
        use models::Booking;
        use uuid::Uuid;
        
        let booking = Booking::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            Local::now(),
            Local::now() + Duration::hours(8),
            "Test".to_string(),
            5,
        );

        let standard = StandardPricing;
        let discount = DiscountPricing;
        
        let std_price = standard.calculate_price(50.0, 8.0, &booking);
        let disc_price = discount.calculate_price(50.0, 8.0, &booking);
        
        assert_eq!(std_price, 400.0);
        assert_eq!(disc_price, 320.0); // 20% discount for 8+ hours
    }

    #[test]
    fn test_adapter_pattern() {
        use adapter::{CalendarAdapter, GoogleCalendarAPI};
        use models::Booking;
        use uuid::Uuid;
        
        let adapter = CalendarAdapter::new(GoogleCalendarAPI);
        
        let booking = Booking::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            Local::now(),
            Local::now() + Duration::hours(2),
            "Team Meeting".to_string(),
            5,
        );

        let result = adapter.sync_booking(&booking, "Test Room");
        assert!(result.is_ok());
        assert!(result.unwrap().starts_with("gcal_"));
    }

    #[test]
    fn test_observer_pattern() {
        use observer::{NotificationSystem, EmailNotifier, AuditLogger};
        use models::Booking;
        use uuid::Uuid;
        use std::sync::Arc;
        
        let system = NotificationSystem::new();
        system.subscribe(Arc::new(EmailNotifier::new()));
        system.subscribe(Arc::new(AuditLogger::new()));

        let booking = Booking::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            Local::now(),
            Local::now() + Duration::hours(1),
            "Test".to_string(),
            5,
        );

        // Should not panic - observers are notified
        system.notify_created(&booking);
    }
}
