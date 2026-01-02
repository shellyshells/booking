// =============================================================================
// ROOM MODEL
// =============================================================================
// Represents a bookable room in the reservation system
// =============================================================================

use serde::{Deserialize, Serialize};

/// Room types available in the system
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RoomType {
    Conference,
    Meeting,
    Training,
    Auditorium,
    PrivateOffice,
}

impl RoomType {
    pub fn as_str(&self) -> &str {
        match self {
            RoomType::Conference => "Conference",
            RoomType::Meeting => "Meeting",
            RoomType::Training => "Training",
            RoomType::Auditorium => "Auditorium",
            RoomType::PrivateOffice => "Private Office",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "conference" => Some(RoomType::Conference),
            "meeting" => Some(RoomType::Meeting),
            "training" => Some(RoomType::Training),
            "auditorium" => Some(RoomType::Auditorium),
            "privateoffice" | "private_office" | "private office" => Some(RoomType::PrivateOffice),
            _ => None,
        }
    }
}

/// Equipment available in rooms
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RoomEquipment {
    Projector,
    Whiteboard,
    VideoConference,
    SoundSystem,
    AirConditioning,
    WiFi,
    Computer,
    Phone,
    Microphone,
    Webcam,
    Printer,
    CoffeeMachine,
}

impl RoomEquipment {
    pub fn as_str(&self) -> &str {
        match self {
            RoomEquipment::Projector => "Projector",
            RoomEquipment::Whiteboard => "Whiteboard",
            RoomEquipment::VideoConference => "Video Conference",
            RoomEquipment::SoundSystem => "Sound System",
            RoomEquipment::AirConditioning => "Air Conditioning",
            RoomEquipment::WiFi => "WiFi",
            RoomEquipment::Computer => "Computer",
            RoomEquipment::Phone => "Phone",
            RoomEquipment::Microphone => "Microphone",
            RoomEquipment::Webcam => "Webcam",
            RoomEquipment::Printer => "Printer",
            RoomEquipment::CoffeeMachine => "Coffee Machine",
        }
    }

    pub fn icon(&self) -> &str {
        match self {
            RoomEquipment::Projector => "📽️",
            RoomEquipment::Whiteboard => "📋",
            RoomEquipment::VideoConference => "📹",
            RoomEquipment::SoundSystem => "🔊",
            RoomEquipment::AirConditioning => "❄️",
            RoomEquipment::WiFi => "📶",
            RoomEquipment::Computer => "💻",
            RoomEquipment::Phone => "☎️",
            RoomEquipment::Microphone => "🎤",
            RoomEquipment::Webcam => "📷",
            RoomEquipment::Printer => "🖨️",
            RoomEquipment::CoffeeMachine => "☕",
        }
    }

    pub fn all() -> Vec<RoomEquipment> {
        vec![
            RoomEquipment::Projector,
            RoomEquipment::Whiteboard,
            RoomEquipment::VideoConference,
            RoomEquipment::SoundSystem,
            RoomEquipment::AirConditioning,
            RoomEquipment::WiFi,
            RoomEquipment::Computer,
            RoomEquipment::Phone,
            RoomEquipment::Microphone,
            RoomEquipment::Webcam,
            RoomEquipment::Printer,
            RoomEquipment::CoffeeMachine,
        ]
    }
}

/// Main Room structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Room {
    pub id: String,
    pub name: String,
    pub room_type: RoomType,
    pub capacity: u32,
    pub floor: i32,
    pub equipment: Vec<RoomEquipment>,
    pub hourly_rate: f64,
    pub is_available: bool,
    pub description: Option<String>,
    pub image_url: Option<String>,
}

impl Room {
    pub fn new(
        id: String,
        name: String,
        room_type: RoomType,
        capacity: u32,
        floor: i32,
        equipment: Vec<RoomEquipment>,
        hourly_rate: f64,
    ) -> Self {
        Room {
            id,
            name,
            room_type,
            capacity,
            floor,
            equipment,
            hourly_rate,
            is_available: true,
            description: None,
            image_url: None,
        }
    }

    pub fn with_description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    pub fn with_image(mut self, url: &str) -> Self {
        self.image_url = Some(url.to_string());
        self
    }

    /// Check if room has specific equipment
    pub fn has_equipment(&self, equipment: &RoomEquipment) -> bool {
        self.equipment.contains(equipment)
    }

    /// Get equipment list as strings
    pub fn equipment_list(&self) -> Vec<String> {
        self.equipment.iter().map(|e| e.as_str().to_string()).collect()
    }

    /// Get a summary of the room
    pub fn summary(&self) -> RoomSummary {
        RoomSummary {
            id: self.id.clone(),
            name: self.name.clone(),
            room_type: self.room_type.as_str().to_string(),
            capacity: self.capacity,
            floor: self.floor,
            hourly_rate: self.hourly_rate,
            is_available: self.is_available,
            equipment_count: self.equipment.len(),
        }
    }
}

/// Lightweight room summary for listings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomSummary {
    pub id: String,
    pub name: String,
    pub room_type: String,
    pub capacity: u32,
    pub floor: i32,
    pub hourly_rate: f64,
    pub is_available: bool,
    pub equipment_count: usize,
}

/// Request to create a new room
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRoomRequest {
    pub name: String,
    pub room_type: String,
    pub capacity: u32,
    pub floor: i32,
    pub equipment: Option<Vec<String>>,
    pub hourly_rate: Option<f64>,
    pub description: Option<String>,
}

/// Request to update a room
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRoomRequest {
    pub name: Option<String>,
    pub capacity: Option<u32>,
    pub floor: Option<i32>,
    pub equipment: Option<Vec<String>>,
    pub hourly_rate: Option<f64>,
    pub is_available: Option<bool>,
    pub description: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_room_creation() {
        let room = Room::new(
            "test-id".to_string(),
            "Test Room".to_string(),
            RoomType::Meeting,
            10,
            1,
            vec![RoomEquipment::WiFi, RoomEquipment::Whiteboard],
            25.0,
        );

        assert_eq!(room.name, "Test Room");
        assert_eq!(room.capacity, 10);
        assert!(room.has_equipment(&RoomEquipment::WiFi));
        assert!(!room.has_equipment(&RoomEquipment::Projector));
    }

    #[test]
    fn test_room_type_conversion() {
        assert_eq!(RoomType::from_str("conference"), Some(RoomType::Conference));
        assert_eq!(RoomType::from_str("MEETING"), Some(RoomType::Meeting));
        assert_eq!(RoomType::from_str("invalid"), None);
    }
}
