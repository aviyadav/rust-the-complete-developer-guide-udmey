use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::SqlitePool;
use std::str::FromStr;

pub type DbPool = SqlitePool;

pub async fn create_pool(database_url: &str) -> Result<DbPool, sqlx::Error> {
    let connect_options = SqliteConnectOptions::from_str(database_url)?
        .create_if_missing(true);

    SqlitePoolOptions::new()
        .max_connections(10)
        .min_connections(2)
        .connect_with(connect_options)
        .await
}

pub async fn run_migrations(pool: &DbPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS tasks (
            id          TEXT    PRIMARY KEY NOT NULL,
            title       TEXT    NOT NULL,
            description TEXT,
            completed   INTEGER NOT NULL DEFAULT 0,
            created_at  TEXT    NOT NULL,
            updated_at  TEXT    NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}
