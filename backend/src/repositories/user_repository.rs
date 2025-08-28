use crate::{
    errors::{AppError, Result},
    models::{User, UserListQuery},
};
use chrono::Utc;
use sqlx::SqlitePool;

/// User repository for database operations
#[derive(Clone)]
pub struct UserRepository {
    pool: SqlitePool,
}

impl UserRepository {
    /// Create a new user repository
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Get all users with pagination
    pub async fn find_all(&self, query: UserListQuery) -> Result<Vec<User>> {
        let limit = query.limit.unwrap_or(100);
        let offset = query.offset.unwrap_or(0);

        let users = sqlx::query_as::<_, User>(
            "SELECT id, name, email, created_at, updated_at FROM users 
             ORDER BY created_at DESC 
             LIMIT ? OFFSET ?",
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(users)
    }

    /// Find user by ID
    pub async fn find_by_id(&self, id: &str) -> Result<User> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, name, email, created_at, updated_at FROM users WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::Database)?
        .ok_or_else(|| AppError::NotFound(format!("User with id '{}' not found", id)))?;

        Ok(user)
    }

    /// Find user by email
    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, name, email, created_at, updated_at FROM users WHERE email = ?",
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(user)
    }

    /// Create a new user
    pub async fn create(&self, user: User) -> Result<User> {
        sqlx::query(
            "INSERT INTO users (id, name, email, created_at, updated_at) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(&user.id)
        .bind(&user.name)
        .bind(&user.email)
        .bind(&user.created_at)
        .bind(&user.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(db_err) if db_err.message().contains("UNIQUE") => {
                AppError::Validation("Email already exists".to_string())
            }
            _ => AppError::Database(e),
        })?;

        Ok(user)
    }

    /// Update an existing user
    pub async fn update(&self, id: &str, name: Option<String>, email: Option<String>) -> Result<User> {
        let mut query_parts = vec!["UPDATE users SET updated_at = ?"];
        let mut params = vec![Utc::now().to_rfc3339()];

        if name.is_some() {
            query_parts.push("name = ?");
            params.push(name.unwrap());
        }

        if email.is_some() {
            query_parts.push("email = ?");
            params.push(email.unwrap());
        }

        let query = format!("{} WHERE id = ?", query_parts.join(", "));
        params.push(id.to_string());

        let mut sqlx_query = sqlx::query(&query);
        for param in &params {
            sqlx_query = sqlx_query.bind(param);
        }

        let result = sqlx_query
            .execute(&self.pool)
            .await
            .map_err(|e| match e {
                sqlx::Error::Database(db_err) if db_err.message().contains("UNIQUE") => {
                    AppError::Validation("Email already exists".to_string())
                }
                _ => AppError::Database(e),
            })?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("User with id '{}' not found", id)));
        }

        self.find_by_id(id).await
    }

    /// Delete a user
    pub async fn delete(&self, id: &str) -> Result<()> {
        let result = sqlx::query("DELETE FROM users WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(AppError::Database)?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("User with id '{}' not found", id)));
        }

        Ok(())
    }
}
