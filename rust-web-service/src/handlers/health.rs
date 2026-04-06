use actix_web::{get, HttpResponse, Responder};
use serde_json::json;

#[utoipa::path(
    get,
    path = "/health",
    tag = "health",
    responses(
        (status = 200, description = "Service is healthy",
         body = serde_json::Value,
         example = json!({"status": "ok", "service": "rust-web-service", "version": "0.1.0"}))
    )
)]
#[get("/health")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "status": "ok",
        "service": "rust-web-service",
        "version": env!("CARGO_PKG_VERSION")
    }))
}
