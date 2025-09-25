use axum::{
    extract::multipart::MultipartError,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Debug, Error, ToSchema)]
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
    NotificationFailed(#[from] crate::notifications::error::NotificationError),
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

// Convert from AuthError
impl From<crate::auth::error::AuthError> for AppError {
    fn from(err: crate::auth::error::AuthError) -> Self {
        match err {
            crate::auth::error::AuthError::WrongCredentials => Self::InvalidCredentials,
            crate::auth::error::AuthError::InvalidCredentials => Self::InvalidCredentials,
            crate::auth::error::AuthError::SignUpFail => Self::Internal("Failed to sign up".into()),
            crate::auth::error::AuthError::TokenCreation => {
                Self::Internal("Failed to create token".into())
            }
            crate::auth::error::AuthError::InvalidToken => Self::InvalidToken,
            crate::auth::error::AuthError::EmailTaken => {
                Self::AlreadyExists("Email already taken".into())
            }
            crate::auth::error::AuthError::UsernameTaken => {
                Self::AlreadyExists("Username already taken".into())
            }
            crate::auth::error::AuthError::UserNotFound => Self::NotFound("User not found".into()),
            crate::auth::error::AuthError::AuthenticationFailed => Self::AuthenticationFailed,
            crate::auth::error::AuthError::Conflict(msg) => Self::AlreadyExists(msg),
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

use aws_credential_types::provider::error::CredentialsError;
use aws_sdk_s3::error::SdkError;
use aws_sdk_s3::presigning::PresigningConfigError;
use utoipa::ToSchema;
use zip::result::ZipError;

// Generic handler for all S3 SDK errors
impl<E> From<SdkError<E>> for AppError
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn from(err: SdkError<E>) -> Self {
        match &err {
            SdkError::ConstructionFailure(err) => {
                tracing::error!("S3 client construction error: {err:?}");
                Self::Internal("Failed to construct S3 client".into())
            }
            SdkError::DispatchFailure(err) => {
                tracing::error!("S3 dispatch error: {err:?}");
                Self::Internal("Failed to dispatch S3 request".into())
            }
            SdkError::ResponseError(err) => {
                tracing::error!("S3 response error: {err:?}");
                Self::Internal("S3 response error".into())
            }
            SdkError::ServiceError(service_err) => {
                // Log the full error for debugging
                tracing::error!("S3 service error: {service_err:?}");

                // Check error type using string matching as a fallback
                let err_str = format!("{service_err:?}");

                if err_str.contains("NoSuchKey") || err_str.contains("NoSuchBucket") {
                    Self::NotFound("S3 resource not found".into())
                } else if err_str.contains("AccessDenied") {
                    Self::AccessDenied
                } else if err_str.contains("InvalidBucketName") {
                    Self::Validation("Invalid S3 bucket name".into())
                } else if err_str.contains("BucketAlreadyExists")
                    || err_str.contains("BucketAlreadyOwnedByYou")
                {
                    Self::AlreadyExists("S3 bucket already exists".into())
                } else {
                    Self::Internal(format!("S3 service error: {service_err:?}"))
                }
            }
            SdkError::TimeoutError(_) => Self::Internal("S3 request timed out".into()),
            _ => Self::Internal(format!("Unknown S3 error: {err:?}")),
        }
    }
}

// Handle Presigning-specific errors
impl From<PresigningConfigError> for AppError {
    fn from(err: PresigningConfigError) -> Self {
        tracing::error!("S3 presigning error: {:?}", err);
        Self::Internal(format!("Failed to generate presigned URL: {err}"))
    }
}

// Handle credential errors
impl From<CredentialsError> for AppError {
    fn from(err: CredentialsError) -> Self {
        tracing::error!("AWS credentials error: {:?}", err);
        Self::Internal("Failed to authenticate with AWS".into())
    }
}

// Convert from DbError
impl From<crate::db::error::DbError> for AppError {
    fn from(err: crate::db::error::DbError) -> Self {
        match err {
            crate::db::error::DbError::Database(msg) => {
                Self::Internal(format!("Database operation failed: {msg}"))
            }
            crate::db::error::DbError::NotFound(msg) => Self::NotFound(format!("Not found: {msg}")),
            crate::db::error::DbError::TransactionFailed => {
                Self::Internal("Transaction failed".into())
            }
            crate::db::error::DbError::AlreadyExists(msg) => {
                Self::AlreadyExists(format!("Resource already exists: {msg}"))
            }
            crate::db::error::DbError::NotRecurring => {
                Self::Validation("Event is not recurring".into())
            }
            crate::db::error::DbError::InvalidRecurrenceId => {
                Self::Validation("Invalid recurrence ID".into())
            }
            crate::db::error::DbError::InvalidRRule(rrule) => {
                Self::Validation(format!("Invalid recurrence rule: {rrule}"))
            }
            crate::db::error::DbError::ParseError(error) => Self::Validation(error.to_string()),
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

impl From<ZipError> for AppError {
    fn from(err: ZipError) -> Self {
        Self::Internal(format!("Zip creation error: {err}"))
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        Self::Internal(format!("IO Error: {err}"))
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
