use crate::models::{Event, Reservation, Ticket};
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

pub async fn create_reservation(
    state: web::Data<AppState>,
    reservation: web::Json<Reservation>,
) -> impl Responder {
    let mut conn = match state.pool.acquire().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Database connection failed"),
    };

    let mut tx = match conn.begin().await {
        Ok(tx) => tx,
        Err(_) => return HttpResponse::InternalServerError().body("Transaction start failed"),
    };

    let reservation = match Reservation::create(&mut tx, reservation.into_inner()).await {
        Ok(res) => res,
        Err(_) => {
            let _ = tx.rollback().await;
            return HttpResponse::InternalServerError().body("Failed to create reservation");
        }
    };

    if let Err(_) = tx.commit().await {
        return HttpResponse::InternalServerError().body("Transaction commit failed");
    }

    HttpResponse::Created().json(reservation)
}

pub async fn get_reservation(data: web::Data<AppState>) -> impl Responder {
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
    reservation_id: web::Path<i32>,
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
    reservation_id: web::Path<i32>,
    reservation: web::Json<CreateItem>,
) -> impl Responder {
    let result = sqlx::query!(
        "UPDATE reservations SET name = $1, description = $2 WHERE id = $3 RETURNING id, name, description",
        reservation.name,
        reservation.description,
        *reservation_id
    )
    .fetch_one(&data.pool)
    .await;

    match result {
        Ok(updated_reservation) => HttpResponse::Ok().json(updated_reservation),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

// Delete
pub async fn delete_reservation(
    data: web::Data<AppState>,
    reservation_id: web::Path<i32>,
) -> impl Responder {
    let result = sqlx::query!("DELETE FROM reservations WHERE id = $1", *reservation_id)
        .execute(&data.pool)
        .await;

    match result {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}
