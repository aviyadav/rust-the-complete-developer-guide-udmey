use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::handlers::{health, tasks};
use crate::models::task::{CreateTaskRequest, Task, UpdateTaskRequest};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "rust-web-service",
        version = "0.1.0",
        description = "Production-style REST API built with Actix-Web and SQLite"
    ),
    paths(
        health::health_check,
        tasks::get_all_tasks,
        tasks::get_task,
        tasks::create_task,
        tasks::update_task,
        tasks::delete_task,
    ),
    components(
        schemas(Task, CreateTaskRequest, UpdateTaskRequest)
    ),
    tags(
        (name = "health", description = "Service health"),
        (name = "tasks",  description = "Task CRUD operations")
    )
)]
pub struct ApiDoc;

/// Register all application routes onto a [`web::ServiceConfig`].
/// Called inside the `HttpServer::new` closure so each worker gets the same config.
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    let openapi = ApiDoc::openapi();

    cfg.service(
        SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi),
    )
    .service(health::health_check)
    .service(
        web::scope("/api").service(
            web::scope("/tasks")
                .service(tasks::get_all_tasks)
                .service(tasks::get_task)
                .service(tasks::create_task)
                .service(tasks::update_task)
                .service(tasks::delete_task),
        ),
    );
}
