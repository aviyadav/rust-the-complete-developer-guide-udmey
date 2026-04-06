use chrono::Utc;
use uuid::Uuid;

use crate::db::DbPool;
use crate::models::task::{CreateTaskRequest, Task, UpdateTaskRequest};

pub struct TaskService;

impl TaskService {
    pub async fn find_all(pool: &DbPool) -> Result<Vec<Task>, sqlx::Error> {
        sqlx::query_as::<_, Task>(
            "SELECT id, title, description, completed, created_at, updated_at \
             FROM tasks \
             ORDER BY created_at DESC",
        )
        .fetch_all(pool)
        .await
    }

    pub async fn find_by_id(pool: &DbPool, id: &str) -> Result<Option<Task>, sqlx::Error> {
        sqlx::query_as::<_, Task>(
            "SELECT id, title, description, completed, created_at, updated_at \
             FROM tasks \
             WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(pool)
        .await
    }

    pub async fn create(pool: &DbPool, req: CreateTaskRequest) -> Result<Task, sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        sqlx::query(
            "INSERT INTO tasks (id, title, description, completed, created_at, updated_at) \
             VALUES (?, ?, ?, 0, ?, ?)",
        )
        .bind(&id)
        .bind(&req.title)
        .bind(&req.description)
        .bind(&now)
        .bind(&now)
        .execute(pool)
        .await?;

        sqlx::query_as::<_, Task>(
            "SELECT id, title, description, completed, created_at, updated_at \
             FROM tasks \
             WHERE id = ?",
        )
        .bind(&id)
        .fetch_one(pool)
        .await
    }

    pub async fn update(
        pool: &DbPool,
        id: &str,
        req: UpdateTaskRequest,
    ) -> Result<Option<Task>, sqlx::Error> {
        let existing = match Self::find_by_id(pool, id).await? {
            Some(t) => t,
            None => return Ok(None),
        };

        let now = Utc::now().to_rfc3339();
        let title = req.title.unwrap_or(existing.title);
        let description = req.description.or(existing.description);
        let completed = req.completed.unwrap_or(existing.completed);

        sqlx::query(
            "UPDATE tasks \
             SET title = ?, description = ?, completed = ?, updated_at = ? \
             WHERE id = ?",
        )
        .bind(&title)
        .bind(&description)
        .bind(completed)
        .bind(&now)
        .bind(id)
        .execute(pool)
        .await?;

        Self::find_by_id(pool, id).await
    }

    pub async fn delete(pool: &DbPool, id: &str) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM tasks WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}
