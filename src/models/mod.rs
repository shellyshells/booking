// =============================================================================
// MODELS MODULE
// =============================================================================
// Data structures and types for the Room Reservation System
// =============================================================================

pub mod room;
pub mod reservation;

pub use room::{Room, RoomType, RoomEquipment, RoomSummary, CreateRoomRequest, UpdateRoomRequest};
pub use reservation::{
    Reservation, ReservationStatus, ReservationSummary, 
    CreateReservationRequest, UpdateReservationRequest, ReservationFilter
};
