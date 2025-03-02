use crate::handlers::AppState;
use crate::routes::config;
use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
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
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin() // Tüm kaynaklara izin ver (*)
            .allow_any_method() // Tüm HTTP metodlarına izin ver (GET, POST vs.)
            .allow_any_header() // Tüm başlıklara izin ver
            .supports_credentials() // İsteğe bağlı: Kimlik bilgilerini destekle
            .max_age(3600); // Preflight isteklerinin önbelleğe alınma süresi (saniye)

        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .app_data(app_state.clone())
            .configure(config)
    })
    .bind(format!("{}:{}", HOST, PORT))?
    .run()
    .await
}
