use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::{SqliteConnectOptions, SqlitePool}, Row};
use std::{collections::HashMap, net::SocketAddr};
use tower_http::cors::{Any, CorsLayer};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct User {
    id: String,
    name: String,
    email: String,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Deserialize)]
struct CreateUser {
    name: String,
    email: String,
}

#[derive(Debug, Deserialize)]
struct UpdateUser {
    name: Option<String>,
    email: Option<String>,
}

#[derive(Serialize)]
struct Hello {
    message: String,
}

#[derive(Clone)]
struct AppState {
    db: SqlitePool,
}

async fn hello() -> Json<Hello> {
    Json(Hello {
        message: "Hello from Axum!".to_string(),
    })
}

async fn get_users(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<User>>, StatusCode> {
    let limit = params
        .get("limit")
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(100);
    
    let offset = params
        .get("offset")
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(0);

    let rows = sqlx::query("SELECT id, name, email, created_at, updated_at FROM users LIMIT ? OFFSET ?")
        .bind(limit)
        .bind(offset)
        .fetch_all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let users: Vec<User> = rows
        .iter()
        .map(|row| User {
            id: row.get("id"),
            name: row.get("name"),
            email: row.get("email"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
        .collect();

    Ok(Json(users))
}

async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<User>, StatusCode> {
    let row = sqlx::query("SELECT id, name, email, created_at, updated_at FROM users WHERE id = ?")
        .bind(&id)
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let user = User {
        id: row.get("id"),
        name: row.get("name"),
        email: row.get("email"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    };

    Ok(Json(user))
}

async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<User>, StatusCode> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    sqlx::query("INSERT INTO users (id, name, email, created_at, updated_at) VALUES (?, ?, ?, ?, ?)")
        .bind(&id)
        .bind(&payload.name)
        .bind(&payload.email)
        .bind(&now)
        .bind(&now)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user = User {
        id,
        name: payload.name,
        email: payload.email,
        created_at: now.clone(),
        updated_at: now,
    };

    Ok(Json(user))
}

async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateUser>,
) -> Result<Json<User>, StatusCode> {
    let now = Utc::now().to_rfc3339();
    
    let mut query = "UPDATE users SET updated_at = ?".to_string();
    let mut params = vec![now.clone()];

    if let Some(name) = &payload.name {
        query.push_str(", name = ?");
        params.push(name.clone());
    }

    if let Some(email) = &payload.email {
        query.push_str(", email = ?");
        params.push(email.clone());
    }

    query.push_str(" WHERE id = ?");
    params.push(id.clone());

    let mut sqlx_query = sqlx::query(&query);
    for param in &params {
        sqlx_query = sqlx_query.bind(param);
    }

    sqlx_query
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    get_user(State(state), Path(id)).await
}

async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(&id)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}

async fn init_db(db: &SqlitePool) -> Result<(), sqlx::Error> {
    // Create users table
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
    .execute(db)
    .await?;

    // Check if we need to seed data
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(db)
        .await?;

    if count.0 == 0 {
        // Insert sample data
        sqlx::query(
            r#"
            INSERT INTO users (id, name, email, created_at, updated_at) VALUES
            ('550e8400-e29b-41d4-a716-446655440000', 'Admin User', 'admin@example.com', datetime('now'), datetime('now')),
            ('550e8400-e29b-41d4-a716-446655440001', 'John Doe', 'john@example.com', datetime('now'), datetime('now'))
            "#,
        )
        .execute(db)
        .await?;
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    // Configure SQLite to create the database file if it doesn't exist.
    let options = SqliteConnectOptions::new()
        .filename("backend.db")
        .create_if_missing(true);

    let pool = SqlitePool::connect_with(options).await.expect("Failed to connect to database");

    init_db(&pool)
        .await
        .expect("Failed to initialize database");

    let state = AppState { db: pool };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/hello", get(hello))
        .route("/api/users", get(get_users).post(create_user))
        .route("/api/users/:id", get(get_user).put(update_user).delete(delete_user))
        .layer(cors)
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Backend running on http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
