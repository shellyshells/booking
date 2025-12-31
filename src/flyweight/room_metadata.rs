use crate::models::RoomType;
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::Mutex;
use once_cell::sync::Lazy;

/// Flyweight Pattern - Share immutable room type data
#[derive(Debug, Clone)]
pub struct RoomMetadata {
    pub room_type: RoomType,
    pub description: String,
    pub typical_capacity: u32,
    pub base_hourly_rate: f64,
    pub recommended_amenities: Vec<String>,
}

impl RoomMetadata {
    fn new(
        room_type: RoomType,
        description: String,
        typical_capacity: u32,
        base_hourly_rate: f64,
        recommended_amenities: Vec<String>,
    ) -> Self {
        Self {
            room_type,
            description,
            typical_capacity,
            base_hourly_rate,
            recommended_amenities,
        }
    }
}

static METADATA_FACTORY: Lazy<Mutex<MetadataFactory>> = Lazy::new(|| {
    Mutex::new(MetadataFactory::new())
});

pub struct MetadataFactory {
    metadata: HashMap<RoomType, Arc<RoomMetadata>>,
}

impl MetadataFactory {
    fn new() -> Self {
        let mut factory = Self {
            metadata: HashMap::new(),
        };
        factory.initialize_metadata();
        factory
    }

    fn initialize_metadata(&mut self) {
        let conference = RoomMetadata::new(
            RoomType::ConferenceRoom,
            "Large room for formal meetings and presentations".to_string(),
            20,
            75.0,
            vec![
                "Projector".to_string(),
                "Whiteboard".to_string(),
                "Video Conference".to_string(),
            ],
        );
        self.metadata.insert(RoomType::ConferenceRoom, Arc::new(conference));

        let meeting = RoomMetadata::new(
            RoomType::MeetingRoom,
            "Small to medium room for team meetings".to_string(),
            8,
            40.0,
            vec!["Whiteboard".to_string()],
        );
        self.metadata.insert(RoomType::MeetingRoom, Arc::new(meeting));

        let training = RoomMetadata::new(
            RoomType::TrainingRoom,
            "Spacious room for training sessions and workshops".to_string(),
            25,
            60.0,
            vec!["Projector".to_string(), "Whiteboard".to_string()],
        );
        self.metadata.insert(RoomType::TrainingRoom, Arc::new(training));

        let executive = RoomMetadata::new(
            RoomType::ExecutiveSuite,
            "Premium room for executive meetings".to_string(),
            6,
            150.0,
            vec![
                "Projector".to_string(),
                "Whiteboard".to_string(),
                "Video Conference".to_string(),
                "Coffee Service".to_string(),
            ],
        );
        self.metadata.insert(RoomType::ExecutiveSuite, Arc::new(executive));

        let auditorium = RoomMetadata::new(
            RoomType::Auditorium,
            "Large venue for presentations and events".to_string(),
            100,
            200.0,
            vec![
                "Sound System".to_string(),
                "Projector".to_string(),
                "Video Conference".to_string(),
            ],
        );
        self.metadata.insert(RoomType::Auditorium, Arc::new(auditorium));
    }

    pub fn get_metadata(room_type: RoomType) -> Arc<RoomMetadata> {
        let factory = METADATA_FACTORY.lock();
        factory
            .metadata
            .get(&room_type)
            .expect("Metadata not found")
            .clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flyweight_sharing() {
        let metadata1 = MetadataFactory::get_metadata(RoomType::ConferenceRoom);
        let metadata2 = MetadataFactory::get_metadata(RoomType::ConferenceRoom);
        
        // Verify same instance
        assert!(Arc::ptr_eq(&metadata1, &metadata2));
    }

    #[test]
    fn test_metadata_content() {
        let metadata = MetadataFactory::get_metadata(RoomType::ExecutiveSuite);
        
        assert_eq!(metadata.typical_capacity, 6);
        assert_eq!(metadata.base_hourly_rate, 150.0);
        assert!(!metadata.recommended_amenities.is_empty());
    }
}
