use axum::{routing::get, Router};
use crate::handlers::{
    create_user, delete_user, get_user, get_users, health_check, update_user,
};

/// Application routes
pub fn app_router() -> Router {
    Router::new()
        // Health check
        .route("/api/health", get(health_check))
        // User routes
        .route("/api/users", get(get_users).post(create_user))
        .route(
            "/api/users/:id",
            get(get_user).put(update_user).delete(delete_user),
        )
}
