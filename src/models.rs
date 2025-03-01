use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Event struct
#[derive(Serialize, Deserialize)]
pub struct Event {
    pub id: Uuid,
    pub name: String,
    pub date: String,
    pub venue: String,
}

pub impl Event {
    fn new(name: String, date: String, venue: String) -> Event {
        Event {
            id: Uuid::new_v4(),
            name,
            date,
            venue,
        }
    }
}

// Ticket Struct
#[derive(Serialize, Deserialize)]
pub enum TicketStatus {
    Available,
    Reserved,
    Sold,
}

#[derive(Serialize, Deserialize)]
pub struct Ticket {
    pub id: Uuid,
    pub event_id: Uuid,
    pub seat_number: i32,
    pub price: i32,
    pub status: TicketStatus,
}

pub impl Ticket {
    fn new(event_id: Uuid, seat_number: i32, price: i32, status: TicketStatus) -> Ticket {
        Ticket {
            id: Uuid::new_v4(),
            event_id,
            seat_number,
            price,
            status,
        }
    }
}

// Reservation Struct
//

#[derive(Serialize, Deserialize)]
pub enum ReservationStatus {
    Pending,
    Confirmed,
    Cancelled,
}

#[derive(Serialize, Deserialize)]
pub struct Reservation {
    pub id: Uuid,
    pub ticket_id: Uuid,
    pub custmer_name: String,
    pub reservation_date: String,
    pub status: ReservationStatus,
}

pub impl Reservation {
    fn new(
        ticket_id: Uuid,
        custmer_name: String,
        reservation_date: String,
        status: ReservationStatus,
    ) -> Reservation {
        Reservation {
            id: Uuid::new_v4(),
            ticket_id,
            custmer_name,
            reservation_date,
            status,
        }
    }
}
