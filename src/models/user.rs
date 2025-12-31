use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserRole {
    Employee,
    Manager,
    Administrator,
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRole::Employee => write!(f, "Employee"),
            UserRole::Manager => write!(f, "Manager"),
            UserRole::Administrator => write!(f, "Administrator"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub role: UserRole,
    pub department: String,
}

impl User {
    pub fn new(name: String, email: String, department: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            email,
            role: UserRole::Employee,
            department,
        }
    }

    pub fn with_role(mut self, role: UserRole) -> Self {
        self.role = role;
        self
    }

    pub fn can_book_room(&self) -> bool {
        matches!(self.role, UserRole::Employee | UserRole::Manager | UserRole::Administrator)
    }

    pub fn can_manage_bookings(&self) -> bool {
        matches!(self.role, UserRole::Manager | UserRole::Administrator)
    }

    pub fn can_configure_system(&self) -> bool {
        matches!(self.role, UserRole::Administrator)
    }
}
