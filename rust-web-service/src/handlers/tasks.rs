use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde_json::json;

use crate::db::DbPool;
use crate::models::task::{CreateTaskRequest, UpdateTaskRequest};
use crate::services::task_service::TaskService;

#[utoipa::path(
    get,
    path = "/api/tasks",
    tag = "tasks",
    responses(
        (status = 200, description = "List of all tasks", body = Vec<Task>)
    )
)]
#[get("")]
pub async fn get_all_tasks(pool: web::Data<DbPool>) -> impl Responder {
    match TaskService::find_all(pool.get_ref()).await {
        Ok(tasks) => HttpResponse::Ok().json(tasks),
        Err(e) => {
            log::error!("Failed to fetch tasks: {}", e);
            HttpResponse::InternalServerError()
                .json(json!({"error": "Failed to fetch tasks"}))
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/tasks/{id}",
    tag = "tasks",
    params(
        ("id" = String, Path, description = "Task UUID")
    ),
    responses(
        (status = 200, description = "Task found", body = Task),
        (status = 404, description = "Task not found",
         body = serde_json::Value,
         example = json!({"error": "Task not found"}))
    )
)]
#[get("/{id}")]
pub async fn get_task(pool: web::Data<DbPool>, path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();
    match TaskService::find_by_id(pool.get_ref(), &id).await {
        Ok(Some(task)) => HttpResponse::Ok().json(task),
        Ok(None) => {
            HttpResponse::NotFound().json(json!({"error": "Task not found"}))
        }
        Err(e) => {
            log::error!("Failed to fetch task {}: {}", id, e);
            HttpResponse::InternalServerError()
                .json(json!({"error": "Failed to fetch task"}))
        }
    }
}

#[utoipa::path(
    post,
    path = "/api/tasks",
    tag = "tasks",
    request_body = CreateTaskRequest,
    responses(
        (status = 201, description = "Task created", body = Task),
        (status = 400, description = "Invalid request",
         body = serde_json::Value,
         example = json!({"error": "title must not be empty"}))
    )
)]
#[post("")]
pub async fn create_task(
    pool: web::Data<DbPool>,
    body: web::Json<CreateTaskRequest>,
) -> impl Responder {
    let req = body.into_inner();
    if req.title.trim().is_empty() {
        return HttpResponse::BadRequest()
            .json(json!({"error": "title must not be empty"}));
    }
    match TaskService::create(pool.get_ref(), req).await {
        Ok(task) => HttpResponse::Created().json(task),
        Err(e) => {
            log::error!("Failed to create task: {}", e);
            HttpResponse::InternalServerError()
                .json(json!({"error": "Failed to create task"}))
        }
    }
}

#[utoipa::path(
    put,
    path = "/api/tasks/{id}",
    tag = "tasks",
    params(
        ("id" = String, Path, description = "Task UUID")
    ),
    request_body = UpdateTaskRequest,
    responses(
        (status = 200, description = "Task updated", body = Task),
        (status = 404, description = "Task not found",
         body = serde_json::Value,
         example = json!({"error": "Task not found"}))
    )
)]
#[put("/{id}")]
pub async fn update_task(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
    body: web::Json<UpdateTaskRequest>,
) -> impl Responder {
    let id = path.into_inner();
    match TaskService::update(pool.get_ref(), &id, body.into_inner()).await {
        Ok(Some(task)) => HttpResponse::Ok().json(task),
        Ok(None) => {
            HttpResponse::NotFound().json(json!({"error": "Task not found"}))
        }
        Err(e) => {
            log::error!("Failed to update task {}: {}", id, e);
            HttpResponse::InternalServerError()
                .json(json!({"error": "Failed to update task"}))
        }
    }
}

#[utoipa::path(
    delete,
    path = "/api/tasks/{id}",
    tag = "tasks",
    params(
        ("id" = String, Path, description = "Task UUID")
    ),
    responses(
        (status = 204, description = "Task deleted"),
        (status = 404, description = "Task not found",
         body = serde_json::Value,
         example = json!({"error": "Task not found"}))
    )
)]
#[delete("/{id}")]
pub async fn delete_task(pool: web::Data<DbPool>, path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();
    match TaskService::delete(pool.get_ref(), &id).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => {
            HttpResponse::NotFound().json(json!({"error": "Task not found"}))
        }
        Err(e) => {
            log::error!("Failed to delete task {}: {}", id, e);
            HttpResponse::InternalServerError()
                .json(json!({"error": "Failed to delete task"}))
        }
    }
}
