use crate::handlers::AppState;
use crate::routes::config;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::PgPool;
use std::env;

mod handlers;
mod models;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to create pool.");

    let app_state = web::Data::new(AppState { pool });

    HttpServer::new(move || App::new().app_data(app_state.clone()).configure(config))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

