use axum::{
    extract::multipart::MultipartError,
    http::StatusCode,
    response::{IntoResponse, Response},
};
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

    // Password handling errors
    #[error("Password hashing error")]
    PasswordHash,

    // General server errors
    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("Bad Request: {0}")]
    BadRequest(String),

    #[error("Notification failed: {0}")]
    NotificationFailed(#[from] ogonek_notifications::NotificationError),
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
            Self::BadRequest(e) => (StatusCode::BAD_REQUEST, e.to_string()),

            Self::NotificationFailed(notification_err) => {
                let status_code = match notification_err.error_code() {
                    4001..=4999 => StatusCode::BAD_REQUEST,
                    5001..=5999 => StatusCode::INTERNAL_SERVER_ERROR,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                };
                return (status_code, notification_err.to_string()).into_response();
            }

            // Password handling errors -> 500
            Self::PasswordHash => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Authentication operation failed".to_string(),
            ),

            // General server errors -> 500
            Self::Internal(details) => {
                tracing::error!("Internal server error: {}", details);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
        };

        (status, message).into_response()
    }
}

use crate::services::AuthError;
// Convert from AuthError
impl From<AuthError> for AppError {
    fn from(err: AuthError) -> Self {
        match err {
            AuthError::WrongCredentials => Self::InvalidCredentials,
            AuthError::InvalidCredentials => Self::InvalidCredentials,
            AuthError::SignUpFail => Self::Internal("Failed to sign up".into()),
            AuthError::TokenCreation => Self::Internal("Failed to create token".into()),
            AuthError::InvalidToken => Self::InvalidToken,
            AuthError::EmailTaken => Self::AlreadyExists("Email already taken".into()),
            AuthError::UsernameTaken => Self::AlreadyExists("Username already taken".into()),
            AuthError::UserNotFound => Self::NotFound("User not found".into()),
            AuthError::AuthenticationFailed => Self::AuthenticationFailed,
            AuthError::Conflict(msg) => Self::AlreadyExists(msg),
        }
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        Self::BadRequest(format!("HTTP client error: {err}"))
    }
}

impl From<axum::http::Error> for AppError {
    fn from(err: axum::http::Error) -> Self {
        Self::BadRequest(format!("HTTP client error: {err}"))
    }
}

impl From<std::env::VarError> for AppError {
    fn from(err: std::env::VarError) -> Self {
        match err {
            std::env::VarError::NotPresent => Self::NotFound("Value not found".into()),
            std::env::VarError::NotUnicode(_) => Self::Internal("Not Unicode".into()),
        }
    }
}

impl From<MultipartError> for AppError {
    fn from(_err: MultipartError) -> Self {
        Self::BadRequest("Multipart Error".into())
    }
}

use ogonek_db::DbError;
// Convert from DbError
impl From<DbError> for AppError {
    fn from(err: DbError) -> Self {
        match err {
            DbError::Database(msg) => Self::Internal(format!("Database operation failed: {msg}")),
            DbError::NotFound(msg) => Self::NotFound(format!("Not found: {msg}")),
            DbError::TransactionFailed => Self::Internal("Transaction failed".into()),
            DbError::AlreadyExists(msg) => {
                Self::AlreadyExists(format!("Resource already exists: {msg}"))
            }
            DbError::NotRecurring => Self::Validation("Event is not recurring".into()),
            DbError::InvalidRecurrenceId => Self::Validation("Invalid recurrence ID".into()),
            DbError::InvalidRRule(rrule) => {
                Self::Validation(format!("Invalid recurrence rule: {rrule}"))
            }
            DbError::ParseError(error) => Self::Validation(error.to_string()),
        }
    }
}
use crate::services::PasswordHashError;
// Convert from PasswordHashError
impl From<PasswordHashError> for AppError {
    fn from(_err: PasswordHashError) -> Self {
        Self::PasswordHash
    }
}

use ogonek_aws::{S3Error, SESError};
// Convert from S3Error
impl From<S3Error> for AppError {
    fn from(err: S3Error) -> Self {
        match err {
            S3Error::AccessDenied => Self::AccessDenied,
            S3Error::NotFound(msg) => Self::NotFound(msg),
            S3Error::Validation(msg) => Self::Validation(msg),
            S3Error::Internal(msg) => Self::Internal(format!("S3 error: {msg}")),
            S3Error::BadRequest(msg) => Self::BadRequest(format!("S3 bad request: {msg}")),
            S3Error::AlreadyExists(msg) => Self::AlreadyExists(msg),
        }
    }
}

impl From<SESError> for AppError {
    fn from(err: SESError) -> Self {
        Self::Internal(err.to_string())
    }
}
// Convert from validator errors
impl From<validator::ValidationErrors> for AppError {
    fn from(errs: validator::ValidationErrors) -> Self {
        Self::Validation(errs.to_string())
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        Self::Internal(format!("IO Error: {err}"))
    }
}
