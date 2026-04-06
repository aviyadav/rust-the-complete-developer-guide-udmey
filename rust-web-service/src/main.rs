use actix_web::{middleware::Logger, web, App, HttpServer};
use std::env;

mod app;
mod db;
mod handlers;
mod middleware;
mod models;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load .env file (silently ignore if missing)
    dotenvy::dotenv().ok();

    // Initialise structured logging; default to "info" if RUST_LOG is not set
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    // --- Database -----------------------------------------------------------
    let database_url =
        env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:tasks.db".to_string());

    let pool = db::create_pool(&database_url)
        .await
        .expect("Failed to create database pool");

    db::run_migrations(&pool)
        .await
        .expect("Failed to run database migrations");

    let pool = web::Data::new(pool);

    // --- Server config -------------------------------------------------------
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "9090".to_string())
        .parse()
        .expect("PORT must be a valid u16");

    let workers: usize = env::var("WORKERS")
        .unwrap_or_else(|_| "4".to_string())
        .parse()
        .expect("WORKERS must be a valid number");

    log::info!("Starting rust-web-service on http://{}:{}", host, port);
    log::info!("Workers: {}", workers);

    // --- HTTP server ---------------------------------------------------------
    HttpServer::new(move || {
        App::new()
            // Shared database pool injected into every handler via web::Data
            .app_data(pool.clone())
            // Built-in access log: "GET /api/tasks HTTP/1.1" 200 … 
            .wrap(Logger::default())
            // Custom middleware: logs method + path + status + elapsed ms
            .wrap(middleware::request_timer::RequestTimer)
            // Routes defined in app.rs
            .configure(app::configure_routes)
    })
    .workers(workers)
    .bind((host.as_str(), port))?
    .run()
    .await
}
