pub mod config;
pub mod logger;
pub mod models;
pub mod factories;
pub mod composite;
pub mod flyweight;
pub mod strategy;
pub mod adapter;
pub mod observer;
pub mod ui;

#[cfg(test)]
mod tests;

pub use models::{Room, Booking, User, BookingStatus};
pub use factories::{RoomFactory, AbstractRoomFactory};
pub use logger::Logger;
