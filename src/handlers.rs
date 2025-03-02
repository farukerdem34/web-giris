use crate::models::{CreateEvent, CreateReservation, CreateTicket, Event, Reservation, Ticket};
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;
#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

fn create_uuid() -> Uuid {
    Uuid::new_v4()
}

// Create
pub async fn create_reservation(
    data: web::Data<AppState>,
    reservation: web::Json<CreateReservation>,
) -> impl Responder {
    let result = sqlx::query!(
        "INSERT INTO reservations (id, ticket_id, customer_name, reservation_date, status) VALUES ($1, $2, $3, $4, $5) RETURNING id, ticket_id, customer_name, reservation_date, status",
        uuid::Uuid::new_v4(),
        reservation.ticket_id,
        reservation.customer_name,
        reservation.reservation_date,
        reservation.status

    )
    .fetch_one(&data.pool)
    .await;

    match result {
        Ok(_record) => HttpResponse::Created(),
        Err(_) => HttpResponse::InternalServerError(),
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
        Ok(_updated_reservation) => HttpResponse::Ok(),
        Err(_) => HttpResponse::NotFound(),
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
        (id, name, date, venue)
        VALUES ($1, $2, $3, $4) 
        RETURNING id, name, date, venue",
        uuid::Uuid::new_v4(),
        event.name,
        event.date,
        event.venue
    )
    .fetch_one(&data.pool)
    .await;

    match result {
        Ok(_record) => HttpResponse::Created(),
        Err(_) => HttpResponse::InternalServerError(),
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
        Ok(_updated_event) => HttpResponse::Ok(),
        Err(_) => HttpResponse::NotFound(),
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
        "INSERT INTO tickets (id, event_id, seat_number, price, status) 
        VALUES ($1, $2, $3, $4,$5) 
        RETURNING id, event_id, seat_number, price, status",
        uuid::Uuid::new_v4(),
        ticket.event_id,
        ticket.seat_number,
        ticket.price,
        ticket.status
    )
    .fetch_one(&data.pool)
    .await;

    match result {
        Ok(_record) => HttpResponse::Created(),
        Err(_) => HttpResponse::InternalServerError(),
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
        Ok(_updated_ticket) => HttpResponse::Ok(),
        Err(_) => HttpResponse::NotFound(),
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
