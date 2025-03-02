use crate::handlers::AppState;
use crate::routes::config;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::env;

mod handlers;
mod models;
mod routes;
const HOST: &str = "0.0.0.0";
const PORT: &str = "8080";
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to create pool.");

    let app_state = web::Data::new(AppState { pool });
    println!("Starting server at http://{}:{}", HOST, PORT);
    HttpServer::new(move || App::new().app_data(app_state.clone()).configure(config))
        .bind(format!("{}:{}", HOST, PORT))?
        .run()
        .await
}
