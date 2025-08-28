use crate::errors::{AppError, Result};
use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};

/// Initialize database connection
pub async fn create_pool(database_url: &str) -> Result<SqlitePool> {
    let options = SqliteConnectOptions::new()
        .filename(database_url)
        .create_if_missing(true);

    let pool = SqlitePool::connect_with(options)
        .await
        .map_err(AppError::Database)?;

    run_migrations(&pool).await?;
    seed_data(&pool).await?;

    Ok(pool)
}

/// Run database migrations
async fn run_migrations(pool: &SqlitePool) -> Result<()> {
    tracing::info!("Running database migrations");

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY NOT NULL,
            name TEXT NOT NULL,
            email TEXT UNIQUE NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await
    .map_err(AppError::Database)?;

    tracing::info!("Database migrations completed");
    Ok(())
}

/// Seed initial data if database is empty
async fn seed_data(pool: &SqlitePool) -> Result<()> {
    // Check if we already have users
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(pool)
        .await
        .map_err(AppError::Database)?;

    if count.0 == 0 {
        tracing::info!("Seeding initial data");

        sqlx::query(
            r#"
            INSERT INTO users (id, name, email, created_at, updated_at) VALUES
            ('550e8400-e29b-41d4-a716-446655440000', 'Admin User', 'admin@example.com', datetime('now'), datetime('now')),
            ('550e8400-e29b-41d4-a716-446655440001', 'John Doe', 'john@example.com', datetime('now'), datetime('now'))
            "#,
        )
        .execute(pool)
        .await
        .map_err(AppError::Database)?;

        tracing::info!("Initial data seeded");
    }

    Ok(())
}
