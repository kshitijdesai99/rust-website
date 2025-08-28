use serde::{Deserialize, Serialize};

/// User model matching backend API
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub created_at: String,
    pub updated_at: String,
}

/// Request to create a new user
#[derive(Debug, Serialize, Clone)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
}

/// Request to update an existing user
#[derive(Debug, Serialize, Clone)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub email: Option<String>,
}

/// Form data for user creation/editing
#[derive(Debug, Clone, Default, PartialEq)]
pub struct UserForm {
    pub name: String,
    pub email: String,
}

impl UserForm {
    /// Create a new empty form
    pub fn new() -> Self {
        Self::default()
    }

    /// Create form from existing user
    pub fn from_user(user: &User) -> Self {
        Self {
            name: user.name.clone(),
            email: user.email.clone(),
        }
    }

    /// Check if form is valid
    pub fn is_valid(&self) -> bool {
        !self.name.trim().is_empty() && !self.email.trim().is_empty() && self.email.contains('@')
    }

    /// Clear the form
    pub fn clear(&mut self) {
        self.name.clear();
        self.email.clear();
    }

    /// Convert to create request
    pub fn to_create_request(&self) -> CreateUserRequest {
        CreateUserRequest {
            name: self.name.trim().to_string(),
            email: self.email.trim().to_string(),
        }
    }

    /// Convert to update request (only includes changed fields)
    pub fn to_update_request(&self) -> UpdateUserRequest {
        UpdateUserRequest {
            name: if self.name.trim().is_empty() { None } else { Some(self.name.trim().to_string()) },
            email: if self.email.trim().is_empty() { None } else { Some(self.email.trim().to_string()) },
        }
    }
}
