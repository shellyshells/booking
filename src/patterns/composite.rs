// =============================================================================
// COMPOSITE PATTERN - Room Groups Management
// =============================================================================
// Problem Solved: Allows treating individual rooms and groups of rooms uniformly.
//                 Users can book entire floors, buildings, or custom room groups
//                 with a single operation, simplifying bulk reservations.
// Location: Used in RoomGroupService for managing room hierarchies
// =============================================================================

use crate::models::room::{Room, RoomType};
use crate::patterns::singleton::log_info;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

// -----------------------------------------------------------------------------
// Reservable Trait - Common interface for rooms and room groups
// -----------------------------------------------------------------------------
pub trait Reservable: Send + Sync {
    fn get_id(&self) -> String;
    fn get_name(&self) -> String;
    fn get_total_capacity(&self) -> u32;
    fn get_hourly_rate(&self) -> f64;
    fn is_available(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> bool;
    fn get_all_room_ids(&self) -> Vec<String>;
    fn get_description(&self) -> String;
}

// Implementation for single Room
impl Reservable for Room {
    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_total_capacity(&self) -> u32 {
        self.capacity
    }

    fn get_hourly_rate(&self) -> f64 {
        self.hourly_rate
    }

    fn is_available(&self, _start: DateTime<Utc>, _end: DateTime<Utc>) -> bool {
        self.is_available
    }

    fn get_all_room_ids(&self) -> Vec<String> {
        vec![self.id.clone()]
    }

    fn get_description(&self) -> String {
        format!(
            "{} ({}) - Capacity: {}, Floor: {}",
            self.name,
            self.room_type.as_str(),
            self.capacity,
            self.floor
        )
    }
}

// -----------------------------------------------------------------------------
// Room Group - Composite node that can contain rooms or other groups
// -----------------------------------------------------------------------------
#[derive(Serialize, Deserialize)]
pub struct RoomGroup {
    pub id: String,
    pub name: String,
    pub description: String,
    #[serde(skip)]
    pub children: Vec<Box<dyn ReservableComponent>>,
    pub child_ids: Vec<String>,        // For serialization
    pub discount_percentage: f64,       // Group booking discount
    pub group_type: RoomGroupType,
}

impl std::fmt::Debug for RoomGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RoomGroup")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("children_count", &self.children.len())
            .finish()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RoomGroupType {
    Floor,
    Building,
    Department,
    Custom,
}

impl RoomGroupType {
    pub fn as_str(&self) -> &str {
        match self {
            RoomGroupType::Floor => "Floor",
            RoomGroupType::Building => "Building",
            RoomGroupType::Department => "Department",
            RoomGroupType::Custom => "Custom Group",
        }
    }
}

// Trait for both leaf and composite nodes
pub trait ReservableComponent: Reservable {
    fn add(&mut self, component: Box<dyn ReservableComponent>) -> Result<(), String>;
    fn remove(&mut self, id: &str) -> Result<(), String>;
    fn is_composite(&self) -> bool;
    fn get_children(&self) -> Vec<&dyn ReservableComponent>;
    fn to_summary(&self) -> ComponentSummary;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentSummary {
    pub id: String,
    pub name: String,
    pub is_group: bool,
    pub total_capacity: u32,
    pub hourly_rate: f64,
    pub room_count: usize,
    pub description: String,
}

// Leaf implementation for Room
impl ReservableComponent for Room {
    fn add(&mut self, _component: Box<dyn ReservableComponent>) -> Result<(), String> {
        Err("Cannot add children to a leaf room".to_string())
    }

    fn remove(&mut self, _id: &str) -> Result<(), String> {
        Err("Cannot remove children from a leaf room".to_string())
    }

    fn is_composite(&self) -> bool {
        false
    }

    fn get_children(&self) -> Vec<&dyn ReservableComponent> {
        vec![]
    }

    fn to_summary(&self) -> ComponentSummary {
        ComponentSummary {
            id: self.id.clone(),
            name: self.name.clone(),
            is_group: false,
            total_capacity: self.capacity,
            hourly_rate: self.hourly_rate,
            room_count: 1,
            description: self.get_description(),
        }
    }
}

impl RoomGroup {
    pub fn new(name: String, description: String, group_type: RoomGroupType) -> Self {
        let id = Uuid::new_v4().to_string();
        log_info(
            &format!("Creating room group: {} ({})", name, group_type.as_str()),
            Some("RoomGroup"),
        );
        
        RoomGroup {
            id,
            name,
            description,
            children: Vec::new(),
            child_ids: Vec::new(),
            discount_percentage: 10.0, // Default 10% discount for groups
            group_type,
        }
    }

    pub fn with_discount(mut self, discount: f64) -> Self {
        self.discount_percentage = discount.clamp(0.0, 50.0);
        self
    }

    /// Calculate total discount based on group size
    pub fn calculate_discount(&self) -> f64 {
        let room_count = self.get_all_room_ids().len();
        let base_discount = self.discount_percentage;
        
        // Additional discount for larger groups
        let size_bonus = match room_count {
            0..=2 => 0.0,
            3..=5 => 2.0,
            6..=10 => 5.0,
            _ => 10.0,
        };
        
        (base_discount + size_bonus).min(50.0)
    }

    /// Get discounted hourly rate
    pub fn get_discounted_rate(&self) -> f64 {
        let base_rate = self.get_hourly_rate();
        let discount = self.calculate_discount();
        base_rate * (1.0 - discount / 100.0)
    }
}

impl Reservable for RoomGroup {
    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_total_capacity(&self) -> u32 {
        self.children
            .iter()
            .map(|c| c.get_total_capacity())
            .sum()
    }

    fn get_hourly_rate(&self) -> f64 {
        self.children
            .iter()
            .map(|c| c.get_hourly_rate())
            .sum()
    }

    fn is_available(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> bool {
        self.children
            .iter()
            .all(|c| c.is_available(start, end))
    }

    fn get_all_room_ids(&self) -> Vec<String> {
        self.children
            .iter()
            .flat_map(|c| c.get_all_room_ids())
            .collect()
    }

    fn get_description(&self) -> String {
        let room_count = self.get_all_room_ids().len();
        format!(
            "{} ({}) - {} rooms, Total capacity: {}, Rate: {:.2}€/hr ({}% discount)",
            self.name,
            self.group_type.as_str(),
            room_count,
            self.get_total_capacity(),
            self.get_discounted_rate(),
            self.calculate_discount()
        )
    }
}

impl ReservableComponent for RoomGroup {
    fn add(&mut self, component: Box<dyn ReservableComponent>) -> Result<(), String> {
        let id = component.get_id();
        
        // Check for duplicate
        if self.child_ids.contains(&id) {
            return Err(format!("Component {} already exists in group", id));
        }
        
        log_info(
            &format!("Adding {} to group {}", component.get_name(), self.name),
            Some("RoomGroup"),
        );
        
        self.child_ids.push(id);
        self.children.push(component);
        Ok(())
    }

    fn remove(&mut self, id: &str) -> Result<(), String> {
        if let Some(pos) = self.child_ids.iter().position(|x| x == id) {
            self.child_ids.remove(pos);
            self.children.remove(pos);
            log_info(
                &format!("Removed component {} from group {}", id, self.name),
                Some("RoomGroup"),
            );
            Ok(())
        } else {
            Err(format!("Component {} not found in group", id))
        }
    }

    fn is_composite(&self) -> bool {
        true
    }

    fn get_children(&self) -> Vec<&dyn ReservableComponent> {
        self.children.iter().map(|c| c.as_ref()).collect()
    }

    fn to_summary(&self) -> ComponentSummary {
        ComponentSummary {
            id: self.id.clone(),
            name: self.name.clone(),
            is_group: true,
            total_capacity: self.get_total_capacity(),
            hourly_rate: self.get_discounted_rate(),
            room_count: self.get_all_room_ids().len(),
            description: self.get_description(),
        }
    }
}

// -----------------------------------------------------------------------------
// Room Group Manager - Handles hierarchical room structures
// -----------------------------------------------------------------------------
pub struct RoomGroupManager {
    groups: HashMap<String, RoomGroup>,
}

impl RoomGroupManager {
    pub fn new() -> Self {
        RoomGroupManager {
            groups: HashMap::new(),
        }
    }

    pub fn create_group(
        &mut self,
        name: String,
        description: String,
        group_type: RoomGroupType,
    ) -> String {
        let group = RoomGroup::new(name, description, group_type);
        let id = group.id.clone();
        self.groups.insert(id.clone(), group);
        id
    }

    pub fn get_group(&self, id: &str) -> Option<&RoomGroup> {
        self.groups.get(id)
    }

    pub fn get_group_mut(&mut self, id: &str) -> Option<&mut RoomGroup> {
        self.groups.get_mut(id)
    }

    pub fn list_groups(&self) -> Vec<ComponentSummary> {
        self.groups.values().map(|g| g.to_summary()).collect()
    }

    pub fn delete_group(&mut self, id: &str) -> Result<(), String> {
        self.groups
            .remove(id)
            .map(|_| ())
            .ok_or_else(|| format!("Group {} not found", id))
    }
}

impl Default for RoomGroupManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::room::RoomEquipment;

    fn create_test_room(name: &str, capacity: u32) -> Room {
        Room::new(
            Uuid::new_v4().to_string(),
            name.to_string(),
            RoomType::Meeting,
            capacity,
            1,
            vec![RoomEquipment::WiFi],
            25.0,
        )
    }

    #[test]
    fn test_room_group_capacity() {
        let mut group = RoomGroup::new(
            "Floor 1".to_string(),
            "All rooms on floor 1".to_string(),
            RoomGroupType::Floor,
        );

        let room1 = create_test_room("Room A", 10);
        let room2 = create_test_room("Room B", 15);

        group.add(Box::new(room1)).unwrap();
        group.add(Box::new(room2)).unwrap();

        assert_eq!(group.get_total_capacity(), 25);
        assert_eq!(group.get_all_room_ids().len(), 2);
    }

    #[test]
    fn test_nested_groups() {
        let mut building = RoomGroup::new(
            "Building A".to_string(),
            "Main building".to_string(),
            RoomGroupType::Building,
        );

        let mut floor1 = RoomGroup::new(
            "Floor 1".to_string(),
            "First floor".to_string(),
            RoomGroupType::Floor,
        );

        floor1.add(Box::new(create_test_room("Room 101", 10))).unwrap();
        floor1.add(Box::new(create_test_room("Room 102", 20))).unwrap();

        building.add(Box::new(floor1)).unwrap();
        building.add(Box::new(create_test_room("Room 001", 5))).unwrap();

        assert_eq!(building.get_total_capacity(), 35);
        assert_eq!(building.get_all_room_ids().len(), 3);
    }
}
