use axum::{extract::State, http::{StatusCode, HeaderMap}, Json};
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter, ActiveModelTrait, Set};
use serde_json::{json, Value};
use crate::{
    auth::{LoginRequest, LoginResponse, UserResponse, create_jwt},
    entities::{users, Users},
    AppState,
};

pub async fn user_login(
    State(state): State<AppState>,
    Json(login_request): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, Json<Value>)> {
    // Demo login via environment-configured credentials
    let expected_email = std::env::var("DEMO_LOGIN_EMAIL").unwrap_or_default();
    let expected_password = std::env::var("DEMO_LOGIN_PASSWORD").unwrap_or_default();
    if !expected_email.is_empty()
        && !expected_password.is_empty()
        && login_request.email == expected_email
        && login_request.password == expected_password
    {
        // Try to find existing user, or create demo user
        let user = Users::find()
            .filter(users::Column::Email.eq(&login_request.email))
            .one(&state.db)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": "Database error"})),
                )
            })?;

        let user = match user {
            Some(user) => user,
            None => {
                // Create demo user if it doesn't exist
                let new_user = users::ActiveModel {
                    email: Set(login_request.email.clone()),
                    name: Set("Demo User".to_string()),
                    ..Default::default()
                };

                new_user.insert(&state.db)
                    .await
                    .map_err(|_| {
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(json!({"error": "Failed to create user"})),
                        )
                    })?
            }
        };

        let token = create_jwt(&user.id.to_string()).map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to create token"})),
            )
        })?;

        Ok(Json(LoginResponse {
            token,
            user: UserResponse {
                id: user.id,
                email: user.email,
                name: user.name,
            },
        }))
    } else {
        Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "Invalid credentials"})),
        ))
    }
}

pub async fn current_user(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<UserResponse>, (StatusCode, Json<Value>)> {
    let claims = crate::auth::check_user_auth(&headers)
        .map_err(|(status, msg)| (status, Json(json!({"error": msg}))))?;
    let user_id: i32 = claims.sub.parse().map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Invalid user ID"})),
        )
    })?;

    let user = Users::find_by_id(user_id)
        .one(&state.db)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Database error"})),
            )
        })?
        .ok_or((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "User not found"})),
        ))?;

    Ok(Json(UserResponse {
        id: user.id,
        email: user.email,
        name: user.name,
    }))
}
