use crate::config::API_BASE_URL;
use crate::models::{CreateUserRequest, UpdateUserRequest, User};
use gloo_net::http::Request;
use serde::Deserialize;

#[derive(Deserialize)]
struct ErrorResponse {
    error: String,
}

/// API client for user operations
pub struct UserApi;

impl UserApi {
    /// Get all users
    pub async fn get_users() -> Result<Vec<User>, String> {
        let url = format!("{}/users", API_BASE_URL);
        
        let response = Request::get(&url)
            .send()
            .await
            .map_err(|e| format!("Request error: {}", e))?;

        if response.ok() {
            response
                .json::<Vec<User>>()
                .await
                .map_err(|e| format!("Parse error: {}", e))
        } else {
            let error_response = response
                .json::<ErrorResponse>()
                .await
                .map_err(|_| "Unknown error occurred".to_string())?;
            Err(error_response.error)
        }
    }

    /// Get user by ID
    pub async fn get_user(id: &str) -> Result<User, String> {
        let url = format!("{}/users/{}", API_BASE_URL, id);
        
        let response = Request::get(&url)
            .send()
            .await
            .map_err(|e| format!("Request error: {}", e))?;

        if response.ok() {
            response
                .json::<User>()
                .await
                .map_err(|e| format!("Parse error: {}", e))
        } else {
            let error_response = response
                .json::<ErrorResponse>()
                .await
                .map_err(|_| "Unknown error occurred".to_string())?;
            Err(error_response.error)
        }
    }

    /// Create a new user
    pub async fn create_user(request: CreateUserRequest) -> Result<User, String> {
        let url = format!("{}/users", API_BASE_URL);
        
        let response = Request::post(&url)
            .json(&request)
            .map_err(|e| format!("Serialization error: {}", e))?
            .send()
            .await
            .map_err(|e| format!("Request error: {}", e))?;

        if response.ok() {
            response
                .json::<User>()
                .await
                .map_err(|e| format!("Parse error: {}", e))
        } else {
            let error_response = response
                .json::<ErrorResponse>()
                .await
                .map_err(|_| "Unknown error occurred".to_string())?;
            Err(error_response.error)
        }
    }

    /// Update an existing user
    pub async fn update_user(id: &str, request: UpdateUserRequest) -> Result<User, String> {
        let url = format!("{}/users/{}", API_BASE_URL, id);
        
        let response = Request::put(&url)
            .json(&request)
            .map_err(|e| format!("Serialization error: {}", e))?
            .send()
            .await
            .map_err(|e| format!("Request error: {}", e))?;

        if response.ok() {
            response
                .json::<User>()
                .await
                .map_err(|e| format!("Parse error: {}", e))
        } else {
            let error_response = response
                .json::<ErrorResponse>()
                .await
                .map_err(|_| "Unknown error occurred".to_string())?;
            Err(error_response.error)
        }
    }

    /// Delete a user
    pub async fn delete_user(id: &str) -> Result<(), String> {
        let url = format!("{}/users/{}", API_BASE_URL, id);
        
        let response = Request::delete(&url)
            .send()
            .await
            .map_err(|e| format!("Request error: {}", e))?;

        if response.ok() {
            Ok(())
        } else {
            let error_response = response
                .json::<ErrorResponse>()
                .await
                .map_err(|_| "Failed to delete user".to_string())?;
            Err(error_response.error)
        }
    }
}
