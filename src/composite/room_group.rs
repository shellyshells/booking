use crate::models::{Bookable, Room};
use chrono::{DateTime, Local};
use uuid::Uuid;

/// Composite Pattern - Manage groups of rooms as a single unit
pub trait RoomComponent: Bookable {
    fn add(&mut self, room: Room) -> Result<(), String>;
    fn remove(&mut self, id: Uuid) -> Result<(), String>;
    fn get_rooms(&self) -> Vec<&Room>;
    fn total_capacity(&self) -> u32;
}

pub struct RoomGroup {
    id: Uuid,
    name: String,
    rooms: Vec<Room>,
}

impl RoomGroup {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            rooms: Vec::new(),
        }
    }

    pub fn book_all(
        &self,
        user_id: Uuid,
        start_time: DateTime<Local>,
        end_time: DateTime<Local>,
    ) -> Vec<crate::models::Booking> {
        use crate::models::Booking;
        
        self.rooms
            .iter()
            .map(|room| {
                Booking::new(
                    room.id,
                    user_id,
                    start_time,
                    end_time,
                    format!("Group booking: {}", self.name),
                    0,
                )
            })
            .collect()
    }

    pub fn find_available_capacity(&self, required_capacity: u32) -> Option<Vec<&Room>> {
        let mut selected_rooms = Vec::new();
        let mut total_cap = 0;

        for room in &self.rooms {
            if total_cap >= required_capacity {
                break;
            }
            selected_rooms.push(room);
            total_cap += room.capacity;
        }

        if total_cap >= required_capacity {
            Some(selected_rooms)
        } else {
            None
        }
    }
}

impl Bookable for RoomGroup {
    fn get_id(&self) -> Uuid {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_capacity(&self) -> u32 {
        self.total_capacity()
    }

    fn is_available(&self) -> bool {
        !self.rooms.is_empty()
    }
}

impl RoomComponent for RoomGroup {
    fn add(&mut self, room: Room) -> Result<(), String> {
        if self.rooms.iter().any(|r| r.id == room.id) {
            return Err("Room already in group".to_string());
        }
        self.rooms.push(room);
        Ok(())
    }

    fn remove(&mut self, id: Uuid) -> Result<(), String> {
        let initial_len = self.rooms.len();
        self.rooms.retain(|r| r.id != id);
        
        if self.rooms.len() < initial_len {
            Ok(())
        } else {
            Err("Room not found in group".to_string())
        }
    }

    fn get_rooms(&self) -> Vec<&Room> {
        self.rooms.iter().collect()
    }

    fn total_capacity(&self) -> u32 {
        self.rooms.iter().map(|r| r.capacity).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::RoomType;

    #[test]
    fn test_room_group_composite() {
        let mut group = RoomGroup::new("Conference Suite".to_string());
        
        let room1 = Room::new("Room A".to_string(), RoomType::MeetingRoom, 10, 1);
        let room2 = Room::new("Room B".to_string(), RoomType::MeetingRoom, 15, 1);
        
        assert!(group.add(room1).is_ok());
        assert!(group.add(room2).is_ok());
        
        assert_eq!(group.total_capacity(), 25);
        assert_eq!(group.get_rooms().len(), 2);
    }

    #[test]
    fn test_find_available_capacity() {
        let mut group = RoomGroup::new("Test Group".to_string());
        
        group.add(Room::new("R1".to_string(), RoomType::MeetingRoom, 10, 1)).unwrap();
        group.add(Room::new("R2".to_string(), RoomType::MeetingRoom, 20, 1)).unwrap();
        
        let result = group.find_available_capacity(25);
        assert!(result.is_some());
        assert_eq!(result.unwrap().len(), 2);
    }
}
