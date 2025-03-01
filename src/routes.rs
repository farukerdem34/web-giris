use crate::handlers::{
    create_event, create_reservation, create_ticket, delete_event, delete_reservation,
    delete_ticket, get_event, get_events, get_reservation, get_reservations, get_ticket,
    get_tickets, update_event, update_reservation, update_ticket,
};
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/reservations")
            .route("", web::post().to(create_reservation))
            .route("", web::get().to(get_reservations))
            .route("/{id}", web::get().to(get_reservation))
            .route("/{id}", web::put().to(update_reservation))
            .route("/{id}", web::delete().to(delete_reservation)),
    )
    .service(
        web::scope("/events")
            .route("", web::post().to(create_event))
            .route("", web::get().to(get_events))
            .route("/{id}", web::get().to(get_event))
            .route("/{id}", web::put().to(update_event))
            .route("/{id}", web::delete().to(delete_event)),
    )
    .service(
        web::scope("/tickets")
            .route("", web::post().to(create_ticket))
            .route("", web::get().to(get_tickets))
            .route("/{id}", web::get().to(get_ticket))
            .route("/{id}", web::put().to(update_ticket))
            .route("/{id}", web::delete().to(delete_ticket)),
    );
}
