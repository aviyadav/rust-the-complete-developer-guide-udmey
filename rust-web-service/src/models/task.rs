use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

/// Represents a task stored in the database.
#[derive(Debug, Serialize, Deserialize, FromRow, Clone, ToSchema)]
pub struct Task {
    /// Unique task identifier (UUID v4)
    pub id: String,
    /// Short title of the task
    pub title: String,
    /// Optional longer description
    pub description: Option<String>,
    /// Whether the task has been completed
    pub completed: bool,
    /// RFC 3339 creation timestamp
    pub created_at: String,
    /// RFC 3339 last-updated timestamp
    pub updated_at: String,
}

/// Request body for creating a new task.
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateTaskRequest {
    /// Short title (required, must not be empty)
    pub title: String,
    /// Optional longer description
    pub description: Option<String>,
}

/// Request body for updating an existing task (all fields optional).
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct UpdateTaskRequest {
    /// New title
    pub title: Option<String>,
    /// New description
    pub description: Option<String>,
    /// Mark as completed or not
    pub completed: Option<bool>,
}
