use crate::models::{CreateEvent, CreateReservation, CreateTicket, Event, Reservation, Ticket};
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;
#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

// Create
pub async fn create_reservation(
    data: web::Data<AppState>,
    reservation: web::Json<CreateReservation>,
) -> impl Responder {
    let result = sqlx::query!(
        "INSERT INTO reservations (ticket_id,customer_name,reservation_date,status) VALUES ($1, $2,$3,$4) RETURNING id, ticket_id,customer_name,reservation_date,status",
        reservation.ticket_id,
        reservation.customer_name,
        reservation.reservation_date,
        reservation.status

    )
    .fetch_one(&data.pool)
    .await;

    match result {
        Ok(record) => HttpResponse::Created().json(record),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_reservations(data: web::Data<AppState>) -> impl Responder {
    let result = sqlx::query_as!(Reservation, "SELECT * FROM reservations")
        .fetch_all(&data.pool)
        .await;

    match result {
        Ok(reservation) => HttpResponse::Ok().json(reservation),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_reservation(
    data: web::Data<AppState>,
    reservation_id: web::Path<Uuid>,
) -> impl Responder {
    let result = sqlx::query_as!(
        Reservation,
        "SELECT * FROM reservations WHERE id = $1",
        *reservation_id
    )
    .fetch_one(&data.pool)
    .await;

    match result {
        Ok(reservation) => HttpResponse::Ok().json(reservation),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub async fn update_reservation(
    data: web::Data<AppState>,
    reservation_id: web::Path<Uuid>,
    reservation: web::Json<CreateReservation>,
) -> impl Responder {
    let result = sqlx::query!(
        "UPDATE reservations SET ticket_id = $1, customer_name= $2,reservation_date=$3,status=$4 
        WHERE id = $5 RETURNING id, ticket_id,customer_name,reservation_date,status",
        reservation.ticket_id,
        reservation.customer_name,
        reservation.reservation_date,
        reservation.status,
        *reservation_id
    )
    .fetch_one(&data.pool)
    .await;

    match result {
        Ok(updated_reservation) => HttpResponse::Ok().json(updated_reservation),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub async fn delete_reservation(
    data: web::Data<AppState>,
    reservation_id: web::Path<Uuid>,
) -> impl Responder {
    let result = sqlx::query!("DELETE FROM reservations WHERE id = $1", *reservation_id)
        .execute(&data.pool)
        .await;

    match result {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

// Event CRUD
// Create
pub async fn create_event(
    data: web::Data<AppState>,
    event: web::Json<CreateEvent>,
) -> impl Responder {
    let result = sqlx::query!(
        "INSERT INTO events 
        (name, date,venue)
        VALUES ($1, $2,$3) 
        RETURNING name, date,venue",
        event.name,
        event.date,
        event.venue
    )
    .fetch_one(&data.pool)
    .await;

    match result {
        Ok(record) => HttpResponse::Created().json(record),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// Read
pub async fn get_events(data: web::Data<AppState>) -> impl Responder {
    let result = sqlx::query_as!(Event, "SELECT * FROM events")
        .fetch_all(&data.pool)
        .await;

    match result {
        Ok(events) => HttpResponse::Ok().json(events),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_event(data: web::Data<AppState>, event_id: web::Path<Uuid>) -> impl Responder {
    let result = sqlx::query_as!(Event, "SELECT * FROM events WHERE id = $1", *event_id)
        .fetch_one(&data.pool)
        .await;

    match result {
        Ok(event) => HttpResponse::Ok().json(event),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

// Update
pub async fn update_event(
    data: web::Data<AppState>,
    event_id: web::Path<Uuid>,
    event: web::Json<CreateEvent>,
) -> impl Responder {
    let result = sqlx::query!(
        "UPDATE events 
        SET name = $1, date = $2, venue = $3 
        WHERE id = $4 
        RETURNING id, name, venue",
        event.name,
        event.date,
        event.venue,
        *event_id
    )
    .fetch_one(&data.pool)
    .await;

    match result {
        Ok(updated_event) => HttpResponse::Ok().json(updated_event),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

// Delete
pub async fn delete_event(data: web::Data<AppState>, event_id: web::Path<Uuid>) -> impl Responder {
    let result = sqlx::query!("DELETE FROM events WHERE id = $1", *event_id)
        .execute(&data.pool)
        .await;

    match result {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

// Ticket CRUD
// Create
pub async fn create_ticket(
    data: web::Data<AppState>,
    ticket: web::Json<CreateTicket>,
) -> impl Responder {
    let result = sqlx::query!(
        "INSERT INTO tickets (event_id, seat_number, price, status) 
        VALUES ($1, $2,$3,$4) 
        RETURNING id, event_id, seat_number, price, status",
        ticket.event_id,
        ticket.seat_number,
        ticket.price,
        ticket.status
    )
    .fetch_one(&data.pool)
    .await;

    match result {
        Ok(record) => HttpResponse::Created().json(record),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// Read
pub async fn get_tickets(data: web::Data<AppState>) -> impl Responder {
    let result = sqlx::query_as!(Ticket, "SELECT * FROM tickets")
        .fetch_all(&data.pool)
        .await;

    match result {
        Ok(tickets) => HttpResponse::Ok().json(tickets),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_ticket(data: web::Data<AppState>, ticket_id: web::Path<Uuid>) -> impl Responder {
    let result = sqlx::query_as!(Ticket, "SELECT * FROM tickets WHERE id = $1", *ticket_id)
        .fetch_one(&data.pool)
        .await;

    match result {
        Ok(ticket) => HttpResponse::Ok().json(ticket),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

// Update
pub async fn update_ticket(
    data: web::Data<AppState>,
    ticket_id: web::Path<Uuid>,
    ticket: web::Json<CreateTicket>,
) -> impl Responder {
    let result = sqlx::query!(
        "UPDATE tickets 
        SET event_id = $1, seat_number = $2, price = $3, status = $4
        WHERE id = $5 RETURNING id, event_id, seat_number, price, status",
        ticket.event_id,
        ticket.seat_number,
        ticket.price,
        ticket.status,
        *ticket_id
    )
    .fetch_one(&data.pool)
    .await;

    match result {
        Ok(updated_ticket) => HttpResponse::Ok().json(updated_ticket),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

// Delete
pub async fn delete_ticket(
    data: web::Data<AppState>,
    ticket_id: web::Path<Uuid>,
) -> impl Responder {
    let result = sqlx::query!("DELETE FROM tickets WHERE id = $1", *ticket_id)
        .execute(&data.pool)
        .await;

    match result {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}
