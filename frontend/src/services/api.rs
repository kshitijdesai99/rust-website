use crate::config::API_BASE_URL;
use crate::models::{CreateUserRequest, UpdateUserRequest, User};
use gloo_net::http::Request;

/// API client for user operations
pub struct UserApi;

impl UserApi {
    /// Get all users
    pub async fn get_users() -> Result<Vec<User>, String> {
        let url = format!("{}/users", API_BASE_URL);
        
        Request::get(&url)
            .send()
            .await
            .map_err(|e| format!("Request error: {}", e))?
            .json::<Vec<User>>()
            .await
            .map_err(|e| format!("Parse error: {}", e))
    }

    /// Get user by ID
    pub async fn get_user(id: &str) -> Result<User, String> {
        let url = format!("{}/users/{}", API_BASE_URL, id);
        
        Request::get(&url)
            .send()
            .await
            .map_err(|e| format!("Request error: {}", e))?
            .json::<User>()
            .await
            .map_err(|e| format!("Parse error: {}", e))
    }

    /// Create a new user
    pub async fn create_user(request: CreateUserRequest) -> Result<User, String> {
        let url = format!("{}/users", API_BASE_URL);
        
        Request::post(&url)
            .json(&request)
            .map_err(|e| format!("Serialization error: {}", e))?
            .send()
            .await
            .map_err(|e| format!("Request error: {}", e))?
            .json::<User>()
            .await
            .map_err(|e| format!("Parse error: {}", e))
    }

    /// Update an existing user
    pub async fn update_user(id: &str, request: UpdateUserRequest) -> Result<User, String> {
        let url = format!("{}/users/{}", API_BASE_URL, id);
        
        Request::put(&url)
            .json(&request)
            .map_err(|e| format!("Serialization error: {}", e))?
            .send()
            .await
            .map_err(|e| format!("Request error: {}", e))?
            .json::<User>()
            .await
            .map_err(|e| format!("Parse error: {}", e))
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
            Err("Failed to delete user".to_string())
        }
    }
}
