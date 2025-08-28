use crate::{
    errors::{AppError, Result},
    models::{CreateUserRequest, UpdateUserRequest, User, UserListQuery, UserResponse},
    repositories::UserRepository,
};
use validator::Validate;

/// User service for business logic
#[derive(Clone)]
pub struct UserService {
    repository: UserRepository,
}

impl UserService {
    /// Create a new user service
    pub fn new(repository: UserRepository) -> Self {
        Self { repository }
    }

    /// Get all users with pagination
    pub async fn get_users(&self, query: UserListQuery) -> Result<Vec<UserResponse>> {
        let users = self.repository.find_all(query).await?;
        Ok(users.into_iter().map(UserResponse::from).collect())
    }

    /// Get user by ID
    pub async fn get_user(&self, id: &str) -> Result<UserResponse> {
        let user = self.repository.find_by_id(id).await?;
        Ok(UserResponse::from(user))
    }

    /// Create a new user
    pub async fn create_user(&self, request: CreateUserRequest) -> Result<UserResponse> {
        // Validate input
        request.validate().map_err(|e| {
            AppError::Validation(format!("Validation failed: {}", e))
        })?;

        // Check if email already exists
        if let Ok(Some(_)) = self.repository.find_by_email(&request.email).await {
            return Err(AppError::Validation("Email already exists".to_string()));
        }

        // Create new user
        let user = User::new(request.name, request.email);
        let created_user = self.repository.create(user).await?;
        
        tracing::info!("Created user with id: {}", created_user.id);
        Ok(UserResponse::from(created_user))
    }

    /// Update an existing user
    pub async fn update_user(&self, id: &str, request: UpdateUserRequest) -> Result<UserResponse> {
        // Validate input
        request.validate().map_err(|e| {
            AppError::Validation(format!("Validation failed: {}", e))
        })?;

        // Check if user exists
        self.repository.find_by_id(id).await?;

        // If email is being updated, check for uniqueness
        if let Some(ref email) = request.email {
            if let Ok(Some(existing_user)) = self.repository.find_by_email(email).await {
                if existing_user.id != id {
                    return Err(AppError::Validation("Email already exists".to_string()));
                }
            }
        }

        let updated_user = self.repository.update(id, request.name, request.email).await?;
        
        tracing::info!("Updated user with id: {}", id);
        Ok(UserResponse::from(updated_user))
    }

    /// Delete a user
    pub async fn delete_user(&self, id: &str) -> Result<()> {
        self.repository.delete(id).await?;
        tracing::info!("Deleted user with id: {}", id);
        Ok(())
    }
}
