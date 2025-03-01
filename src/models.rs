use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// Event struct
#[derive(Serialize, Deserialize, FromRow)]
pub struct Event {
    pub id: Uuid,
    pub name: String,
    pub date: String,
    pub venue: String,
}

#[derive(Deserialize)]
pub struct CreateEvent {
    pub name: String,
    pub date: String,
    pub venue: String,
}
// Ticket Struct

#[derive(Serialize, Deserialize, FromRow)]
pub struct Ticket {
    pub id: Uuid,
    pub event_id: Uuid,
    pub seat_number: i32,
    pub price: i32,
    pub status: String,
}

#[derive(Deserialize)]
pub struct CreateTicket {
    pub event_id: Uuid,
    pub seat_number: i32,
    pub price: i32,
    pub status: String,
}

// Reservation Struct
#[derive(Serialize, Deserialize, FromRow)]
pub struct Reservation {
    pub id: Uuid,
    pub ticket_id: Uuid,
    pub customer_name: String,
    pub reservation_date: String,
    pub status: String,
}

#[derive(Deserialize)]
pub struct CreateReservation {
    pub ticket_id: Uuid,
    pub customer_name: String,
    pub reservation_date: String,
    pub status: String,
}
