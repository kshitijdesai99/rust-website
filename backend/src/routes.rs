use axum::{routing::get, Router};
use crate::handlers::{
    create_user, delete_user, get_user, get_users, health_check, update_user,
};
use crate::services::UserService;

/// Application routes
pub fn app_router(state: UserService) -> Router {
    Router::new()
        .route("/api/health", get(health_check))
        .route("/api/users", get(get_users).post(create_user))
        .route(
            "/api/users/:id",
            get(get_user).put(update_user).delete(delete_user),
        )
        .with_state(state)
}
