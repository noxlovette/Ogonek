// src/error.rs
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    // Authentication errors
    #[error("Invalid credentials")]
    InvalidCredentials,
    
    #[error("Authentication failed")]
    AuthenticationFailed,
    
    #[error("Access denied")]
    AccessDenied,
    
    #[error("Invalid token")]
    InvalidToken,
    
    // Resource errors
    #[error("Resource not found: {0}")]
    NotFound(String),
    
    #[error("Resource already exists: {0}")]
    AlreadyExists(String),
    
    // Validation errors
    #[error("Validation error: {0}")]
    Validation(String),
    
    // Database errors
    #[error("Database error")]
    Database(#[from] sqlx::Error),
    
    // Password handling errors
    #[error("Password hashing error")]
    PasswordHash,
    
    // General server errors
    #[error("Internal server error: {0}")]
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            // Authentication errors -> 401/403
            Self::InvalidCredentials => (StatusCode::UNAUTHORIZED, self.to_string()),
            Self::AuthenticationFailed => (StatusCode::UNAUTHORIZED, self.to_string()),
            Self::AccessDenied => (StatusCode::FORBIDDEN, self.to_string()),
            Self::InvalidToken => (StatusCode::UNAUTHORIZED, self.to_string()),
            
            // Resource errors -> 404/409
            Self::NotFound(_resource) => (StatusCode::NOT_FOUND, self.to_string()),
            Self::AlreadyExists(_resource) => (StatusCode::CONFLICT, self.to_string()),
            
            // Validation errors -> 400
            Self::Validation(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            
            // Database errors -> 500 (or map specific DB errors to more appropriate codes)
            Self::Database(db_err) => {
                // Log detailed DB error for internal visibility
                tracing::error!("Database error: {:?}", db_err);
                
                // Map certain DB errors to specific status codes
                if let sqlx::Error::Database(dbe) = db_err {
                    if let Some(constraint) = dbe.constraint() {
                        if constraint.contains("_key") || constraint.contains("_unique") {
                            return Self::AlreadyExists(format!("Resource with this property already exists")).into_response();
                        }
                    }
                }
                (StatusCode::INTERNAL_SERVER_ERROR, "Database operation failed".to_string())
            }
            
            // Password handling errors -> 500
            Self::PasswordHash => (StatusCode::INTERNAL_SERVER_ERROR, "Authentication operation failed".to_string()),
            
            // General server errors -> 500
            Self::Internal(details) => {
                tracing::error!("Internal server error: {}", details);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            }
        };
        
        // Create a consistent error response format
        let body = Json(json!({
            "error": {
                "message": message,
                "code": status.as_u16()
            }
        }));
        
        (status, body).into_response()
    }
}


// Convert from AuthError
impl From<crate::auth::error::AuthError> for AppError {
    fn from(err: crate::auth::error::AuthError) -> Self {
        match err {
            crate::auth::error::AuthError::WrongCredentials => Self::InvalidCredentials,
            crate::auth::error::AuthError::InvalidCredentials => Self::InvalidCredentials,
            crate::auth::error::AuthError::SignUpFail => Self::Internal("Failed to sign up".into()),
            crate::auth::error::AuthError::TokenCreation => Self::Internal("Failed to create token".into()),
            crate::auth::error::AuthError::InvalidToken => Self::InvalidToken,
            crate::auth::error::AuthError::EmailTaken => Self::AlreadyExists("Email already taken".into()),
            crate::auth::error::AuthError::UsernameTaken => Self::AlreadyExists("Username already taken".into()),
            crate::auth::error::AuthError::UserNotFound => Self::NotFound("User not found".into()),
            crate::auth::error::AuthError::AuthenticationFailed => Self::AuthenticationFailed,
            crate::auth::error::AuthError::Conflict(msg) => Self::AlreadyExists(msg),
        }
    }
}

// Convert from DbError
impl From<crate::db::error::DbError> for AppError {
    fn from(err: crate::db::error::DbError) -> Self {
        match err {
            crate::db::error::DbError::Db => Self::Internal("Database operation failed".into()),
            crate::db::error::DbError::NotFound => Self::NotFound("Resource not found".into()),
            crate::db::error::DbError::TransactionFailed => Self::Internal("Transaction failed".into()),
            crate::db::error::DbError::AlreadyExists => Self::AlreadyExists("Resource already exists".into()),
        }
    }
}

// Convert from PasswordHashError
impl From<crate::auth::error::PasswordHashError> for AppError {
    fn from(_err: crate::auth::error::PasswordHashError) -> Self {
        Self::PasswordHash
    }
}

// Convert from validator errors
impl From<validator::ValidationErrors> for AppError {
    fn from(errs: validator::ValidationErrors) -> Self {
        Self::Validation(errs.to_string())
    }
}

// result extention trait
pub trait ResultExt<T, E> {
    fn context(self, context: impl Into<String>) -> Result<T, AppError>;
}

impl<T, E: Into<AppError>> ResultExt<T, E> for Result<T, E> {
    fn context(self, context: impl Into<String>) -> Result<T, AppError> {
        self.map_err(|err| {
            let app_err = err.into();
            // Optionally log or modify the error based on context
            tracing::debug!("{}: {:?}", context.into(), app_err);
            app_err
        })
    }
}