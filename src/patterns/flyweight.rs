// =============================================================================
// FLYWEIGHT PATTERN - Room Type Information Sharing
// =============================================================================
// Problem Solved: Shares immutable data about room types across many room
//                 instances, reducing memory usage when there are many rooms.
//                 Room type descriptions, icons, and default settings are shared.
// Location: Used by RoomFactory and Room display components
// =============================================================================

use crate::models::room::{RoomEquipment, RoomType};
use crate::patterns::singleton::log_info;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

// -----------------------------------------------------------------------------
// Shared Room Type Information (Intrinsic State)
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomTypeInfo {
    pub room_type: RoomType,
    pub display_name: String,
    pub description: String,
    pub icon: String,                      // CSS icon class or emoji
    pub color: String,                     // Theme color for UI
    pub default_equipment: Vec<RoomEquipment>,
    pub base_hourly_rate: f64,
    pub min_capacity: u32,
    pub max_capacity: u32,
    pub typical_use_cases: Vec<String>,
    pub amenities_description: String,
}

impl RoomTypeInfo {
    pub fn new(
        room_type: RoomType,
        display_name: String,
        description: String,
        icon: String,
        color: String,
        default_equipment: Vec<RoomEquipment>,
        base_hourly_rate: f64,
        min_capacity: u32,
        max_capacity: u32,
        typical_use_cases: Vec<String>,
    ) -> Self {
        let amenities_description = default_equipment
            .iter()
            .map(|e| e.as_str())
            .collect::<Vec<_>>()
            .join(", ");

        RoomTypeInfo {
            room_type,
            display_name,
            description,
            icon,
            color,
            default_equipment,
            base_hourly_rate,
            min_capacity,
            max_capacity,
            typical_use_cases,
            amenities_description,
        }
    }
}

// -----------------------------------------------------------------------------
// Flyweight Factory - Creates and caches room type information
// -----------------------------------------------------------------------------
pub struct RoomTypeFlyweight {
    room_types: HashMap<RoomType, Arc<RoomTypeInfo>>,
}

impl RoomTypeFlyweight {
    fn new() -> Self {
        log_info(
            "Initializing RoomTypeFlyweight with shared room type data",
            Some("Flyweight"),
        );

        let mut room_types = HashMap::new();

        // Conference Room Type
        room_types.insert(
            RoomType::Conference,
            Arc::new(RoomTypeInfo::new(
                RoomType::Conference,
                "Conference Room".to_string(),
                "Large rooms designed for formal meetings, presentations, and video conferences with clients or stakeholders.".to_string(),
                "🏢".to_string(),
                "#3B82F6".to_string(), // Blue
                vec![
                    RoomEquipment::Projector,
                    RoomEquipment::Whiteboard,
                    RoomEquipment::VideoConference,
                    RoomEquipment::AirConditioning,
                    RoomEquipment::WiFi,
                ],
                50.0,
                10,
                50,
                vec![
                    "Client presentations".to_string(),
                    "Board meetings".to_string(),
                    "Video conferences".to_string(),
                    "Training sessions".to_string(),
                ],
            )),
        );

        // Meeting Room Type
        room_types.insert(
            RoomType::Meeting,
            Arc::new(RoomTypeInfo::new(
                RoomType::Meeting,
                "Meeting Room".to_string(),
                "Compact rooms ideal for team discussions, brainstorming sessions, and small group collaborations.".to_string(),
                "👥".to_string(),
                "#10B981".to_string(), // Green
                vec![
                    RoomEquipment::Whiteboard,
                    RoomEquipment::WiFi,
                    RoomEquipment::AirConditioning,
                ],
                25.0,
                2,
                12,
                vec![
                    "Team meetings".to_string(),
                    "Brainstorming".to_string(),
                    "One-on-ones".to_string(),
                    "Quick syncs".to_string(),
                ],
            )),
        );

        // Training Room Type
        room_types.insert(
            RoomType::Training,
            Arc::new(RoomTypeInfo::new(
                RoomType::Training,
                "Training Room".to_string(),
                "Rooms equipped for educational sessions, workshops, and hands-on learning experiences.".to_string(),
                "📚".to_string(),
                "#8B5CF6".to_string(), // Purple
                vec![
                    RoomEquipment::Projector,
                    RoomEquipment::Whiteboard,
                    RoomEquipment::Computer,
                    RoomEquipment::AirConditioning,
                    RoomEquipment::WiFi,
                ],
                40.0,
                8,
                30,
                vec![
                    "Employee training".to_string(),
                    "Workshops".to_string(),
                    "Technical demos".to_string(),
                    "Onboarding sessions".to_string(),
                ],
            )),
        );

        // Auditorium Type
        room_types.insert(
            RoomType::Auditorium,
            Arc::new(RoomTypeInfo::new(
                RoomType::Auditorium,
                "Auditorium".to_string(),
                "Large venues for company-wide presentations, events, and gatherings with professional audio-visual equipment.".to_string(),
                "🎭".to_string(),
                "#F59E0B".to_string(), // Amber
                vec![
                    RoomEquipment::Projector,
                    RoomEquipment::SoundSystem,
                    RoomEquipment::VideoConference,
                    RoomEquipment::AirConditioning,
                    RoomEquipment::WiFi,
                    RoomEquipment::Microphone,
                ],
                100.0,
                50,
                500,
                vec![
                    "All-hands meetings".to_string(),
                    "Product launches".to_string(),
                    "Company events".to_string(),
                    "Guest speakers".to_string(),
                ],
            )),
        );

        // Private Office Type
        room_types.insert(
            RoomType::PrivateOffice,
            Arc::new(RoomTypeInfo::new(
                RoomType::PrivateOffice,
                "Private Office".to_string(),
                "Individual workspaces for focused work, confidential calls, or temporary office needs.".to_string(),
                "💼".to_string(),
                "#EC4899".to_string(), // Pink
                vec![
                    RoomEquipment::Computer,
                    RoomEquipment::Phone,
                    RoomEquipment::AirConditioning,
                    RoomEquipment::WiFi,
                ],
                15.0,
                1,
                4,
                vec![
                    "Focused work".to_string(),
                    "Private calls".to_string(),
                    "Guest offices".to_string(),
                    "Interview rooms".to_string(),
                ],
            )),
        );

        RoomTypeFlyweight { room_types }
    }

    /// Get shared room type information (flyweight)
    pub fn get_room_info(&self, room_type: &RoomType) -> RoomTypeInfo {
        self.room_types
            .get(room_type)
            .map(|arc| (**arc).clone())
            .unwrap_or_else(|| {
                log_info(
                    &format!("Unknown room type: {:?}, returning default", room_type),
                    Some("Flyweight"),
                );
                self.get_default_info()
            })
    }

    /// Get reference to shared room type information (more efficient)
    pub fn get_room_info_ref(&self, room_type: &RoomType) -> Option<Arc<RoomTypeInfo>> {
        self.room_types.get(room_type).cloned()
    }

    /// Get all room type information
    pub fn get_all_types(&self) -> Vec<RoomTypeInfo> {
        self.room_types
            .values()
            .map(|arc| (**arc).clone())
            .collect()
    }

    /// Get room types suitable for a given capacity
    pub fn get_types_for_capacity(&self, capacity: u32) -> Vec<RoomTypeInfo> {
        self.room_types
            .values()
            .filter(|info| capacity >= info.min_capacity && capacity <= info.max_capacity)
            .map(|arc| (**arc).clone())
            .collect()
    }

    fn get_default_info(&self) -> RoomTypeInfo {
        RoomTypeInfo::new(
            RoomType::Meeting,
            "Unknown Room".to_string(),
            "Room type information not available.".to_string(),
            "❓".to_string(),
            "#6B7280".to_string(),
            vec![RoomEquipment::WiFi],
            20.0,
            1,
            10,
            vec!["General use".to_string()],
        )
    }

    /// Calculate statistics about memory savings
    pub fn memory_stats(&self, total_rooms: usize) -> FlyweightStats {
        let shared_data_size = std::mem::size_of::<RoomTypeInfo>() * self.room_types.len();
        let without_flyweight = std::mem::size_of::<RoomTypeInfo>() * total_rooms;
        let savings = if total_rooms > 0 {
            ((without_flyweight - shared_data_size) as f64 / without_flyweight as f64) * 100.0
        } else {
            0.0
        };

        FlyweightStats {
            shared_instances: self.room_types.len(),
            total_rooms,
            shared_data_bytes: shared_data_size,
            estimated_savings_bytes: without_flyweight.saturating_sub(shared_data_size),
            savings_percentage: savings,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlyweightStats {
    pub shared_instances: usize,
    pub total_rooms: usize,
    pub shared_data_bytes: usize,
    pub estimated_savings_bytes: usize,
    pub savings_percentage: f64,
}

// Global flyweight instance
pub static ROOM_TYPE_FLYWEIGHT: Lazy<RoomTypeFlyweight> = Lazy::new(RoomTypeFlyweight::new);

// Convenience functions
pub fn get_room_type_info(room_type: &RoomType) -> RoomTypeInfo {
    ROOM_TYPE_FLYWEIGHT.get_room_info(room_type)
}

pub fn get_all_room_types() -> Vec<RoomTypeInfo> {
    ROOM_TYPE_FLYWEIGHT.get_all_types()
}

pub fn get_room_types_for_capacity(capacity: u32) -> Vec<RoomTypeInfo> {
    ROOM_TYPE_FLYWEIGHT.get_types_for_capacity(capacity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flyweight_sharing() {
        let info1 = ROOM_TYPE_FLYWEIGHT.get_room_info_ref(&RoomType::Conference);
        let info2 = ROOM_TYPE_FLYWEIGHT.get_room_info_ref(&RoomType::Conference);

        // Both should point to the same Arc
        assert!(Arc::ptr_eq(&info1.unwrap(), &info2.unwrap()));
    }

    #[test]
    fn test_all_types_available() {
        let types = get_all_room_types();
        assert_eq!(types.len(), 5);
    }

    #[test]
    fn test_capacity_filtering() {
        let small_room_types = get_room_types_for_capacity(2);
        assert!(small_room_types.iter().any(|t| t.room_type == RoomType::Meeting));
        assert!(small_room_types.iter().any(|t| t.room_type == RoomType::PrivateOffice));
    }

    #[test]
    fn test_memory_stats() {
        let stats = ROOM_TYPE_FLYWEIGHT.memory_stats(100);
        assert!(stats.savings_percentage > 0.0);
        println!("Flyweight memory savings: {:.1}%", stats.savings_percentage);
    }
}
