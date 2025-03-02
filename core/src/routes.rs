use crate::handlers::{
    create_event, create_reservation, create_ticket, create_user, delete_event, delete_reservation,
    delete_ticket, delete_user, get_event, get_events, get_reservation, get_reservations,
    get_ticket, get_tickets, get_user, get_users, update_event, update_reservation, update_ticket,
    update_user,
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
    )
    .service(
        web::scope("/users")
            .route("", web::post().to(create_user))
            .route("", web::get().to(get_users))
            .route("/{id}", web::get().to(get_user))
            .route("/{id}", web::put().to(update_user))
            .route("/{id}", web::delete().to(delete_user)),
    );
}
