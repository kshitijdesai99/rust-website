use axum::{
    routing::{get, post, get_service},
    Router,
    Json,
};
use serde_json::{json, Value};
use tower::ServiceBuilder;
use tower_http::{
    cors::{CorsLayer, Any},
    services::{ServeDir, ServeFile},
};
use chrono;
use crate::{admin, api::handlers, AppState};

pub fn create_routes(state: AppState) -> Router {
    Router::new()
        // Health endpoints
        .route("/health", get(health_check))
        .route("/api/v1/status", get(api_status))
        
        // Authentication endpoints
        .route("/api/auth/login", post(handlers::user_login))
        .route("/api/user/current", get(handlers::current_user))
        
        // Admin panel
        .route("/api/admin/config", get(admin::admin_panel_config))
        .route("/login", get(admin::admin_login_page))
        
        // GraphQL endpoints
        .route("/api/graphql", get(admin::graphql_playground))
        .route("/api/graphql", post(admin::graphql_handler))
        
        // Blog read-only REST endpoints
        .route("/api/blogs", get(handlers::list_blogs))
        .route("/api/blogs/{slug}", get(handlers::get_blog_by_slug))
        
        // Serve admin panel static files
        .nest_service(
            "/admin",
            get_service(
                ServeDir::new("assets/admin")
                    .fallback(ServeFile::new("assets/admin/index.html"))
            )
        )
        
        // Apply middleware
        .layer(
            ServiceBuilder::new()
                .layer(
                    CorsLayer::new()
                        .allow_origin(Any)
                        .allow_methods(Any)
                        .allow_headers(Any)
                )
        )
        .with_state(state)
}

async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now()
    }))
}

async fn api_status() -> Json<Value> {
    Json(json!({
        "service": "backend",
        "version": "0.1.0",
        "status": "running"
    }))
}