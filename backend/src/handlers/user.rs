use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use crate::{
    errors::Result,
    models::{CreateUserRequest, UpdateUserRequest, UserListQuery, UserResponse},
    services::UserService,
};

/// Get all users
pub async fn get_users(
    State(user_service): State<UserService>,
    Query(params): Query<UserListQuery>,
) -> Result<Json<Vec<UserResponse>>> {
    let users = user_service.get_users(params).await?;
    Ok(Json(users))
}

/// Get user by ID
pub async fn get_user(
    State(user_service): State<UserService>,
    Path(id): Path<String>,
) -> Result<Json<UserResponse>> {
    let user = user_service.get_user(&id).await?;
    Ok(Json(user))
}

/// Create a new user
pub async fn create_user(
    State(user_service): State<UserService>,
    Json(request): Json<CreateUserRequest>,
) -> Result<Json<UserResponse>> {
    let user = user_service.create_user(request).await?;
    Ok(Json(user))
}

/// Update an existing user
pub async fn update_user(
    State(user_service): State<UserService>,
    Path(id): Path<String>,
    Json(request): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>> {
    let user = user_service.update_user(&id, request).await?;
    Ok(Json(user))
}

/// Delete a user
pub async fn delete_user(
    State(user_service): State<UserService>,
    Path(id): Path<String>,
) -> Result<StatusCode> {
    user_service.delete_user(&id).await?;
    Ok(StatusCode::NO_CONTENT)
}
