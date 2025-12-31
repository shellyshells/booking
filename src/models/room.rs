use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RoomType {
    ConferenceRoom,
    MeetingRoom,
    TrainingRoom,
    ExecutiveSuite,
    Auditorium,
}

impl std::fmt::Display for RoomType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RoomType::ConferenceRoom => write!(f, "Conference Room"),
            RoomType::MeetingRoom => write!(f, "Meeting Room"),
            RoomType::TrainingRoom => write!(f, "Training Room"),
            RoomType::ExecutiveSuite => write!(f, "Executive Suite"),
            RoomType::Auditorium => write!(f, "Auditorium"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Room {
    pub id: Uuid,
    pub name: String,
    pub room_type: RoomType,
    pub capacity: u32,
    pub floor: u32,
    pub has_projector: bool,
    pub has_whiteboard: bool,
    pub has_video_conf: bool,
    pub hourly_rate: f64,
}

impl Room {
    pub fn new(
        name: String,
        room_type: RoomType,
        capacity: u32,
        floor: u32,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            room_type,
            capacity,
            floor,
            has_projector: false,
            has_whiteboard: false,
            has_video_conf: false,
            hourly_rate: 50.0,
        }
    }

    pub fn with_amenities(
        mut self,
        projector: bool,
        whiteboard: bool,
        video_conf: bool,
    ) -> Self {
        self.has_projector = projector;
        self.has_whiteboard = whiteboard;
        self.has_video_conf = video_conf;
        self
    }

    pub fn with_rate(mut self, rate: f64) -> Self {
        self.hourly_rate = rate;
        self
    }
}

/// Trait for bookable entities
pub trait Bookable {
    fn get_id(&self) -> Uuid;
    fn get_name(&self) -> &str;
    fn get_capacity(&self) -> u32;
    fn is_available(&self) -> bool;
}

impl Bookable for Room {
    fn get_id(&self) -> Uuid {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_capacity(&self) -> u32 {
        self.capacity
    }

    fn is_available(&self) -> bool {
        true // Would check against booking system
    }
}
