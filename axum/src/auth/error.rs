use crate::db::error::DbError;
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use thiserror::Error;

impl From<sqlx::Error> for AuthError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::RowNotFound => Self::UserNotFound,
            other => {
                eprintln!("Database error: {other}");
                Self::InvalidCredentials
            }
        }
    }
}

impl From<PasswordHashError> for AuthError {
    fn from(error: PasswordHashError) -> Self {
        eprintln!("{error}");
        Self::WrongCredentials
    }
}

impl From<ValidationError> for AuthError {
    fn from(error: ValidationError) -> Self {
        eprintln!("{error}");
        Self::InvalidCredentials
    }
}

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Wrong credentials")]
    WrongCredentials,
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Failed to Sign Up")]
    SignUpFail,
    #[error("Token creation error")]
    TokenCreation,
    #[error("Invalid token")]
    InvalidToken,
    #[error("Email already taken")]
    EmailTaken,
    #[error("Username already taken")]
    UsernameTaken,
    #[error("User not found")]
    UserNotFound,
    #[error("Authentication failed")]
    AuthenticationFailed,
    #[error("Conflict: {0}")]
    Conflict(String),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::InvalidCredentials => (StatusCode::BAD_REQUEST, "Invalid credentials"),
            AuthError::SignUpFail => (StatusCode::BAD_REQUEST, "Failed to sign up"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token"),
            AuthError::EmailTaken => (StatusCode::BAD_REQUEST, "Email already taken"),
            AuthError::UsernameTaken => (StatusCode::BAD_REQUEST, "Username already taken"),
            AuthError::UserNotFound => (StatusCode::NOT_FOUND, "User not found"),
            AuthError::AuthenticationFailed => (StatusCode::UNAUTHORIZED, "Authentication failed"),
            AuthError::Conflict(ref message) => (StatusCode::CONFLICT, message.as_str()),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

use argon2::password_hash::Error as Argon2Error;
use validator::ValidationError;

#[derive(Debug, thiserror::Error)]
pub enum PasswordHashError {
    #[error("Failed to hash password: {0}")]
    HashingError(Argon2Error),
    #[error("Password verification failed after hashiong")]
    VerificationError,
}

impl From<Argon2Error> for PasswordHashError {
    fn from(error: Argon2Error) -> Self {
        eprintln!("{error}");
        Self::HashingError(error)
    }
}

impl From<DbError> for PasswordHashError {
    fn from(error: DbError) -> Self {
        eprintln!("{error}");
        Self::VerificationError
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use argon2::password_hash::Error as Argon2Error;
    use sqlx::Error as SqlxError;
    use validator::ValidationError;

    #[test]
    fn test_sqlx_error_conversion_row_not_found() {
        let sqlx_error = SqlxError::RowNotFound;
        let auth_error: AuthError = sqlx_error.into();
        assert!(matches!(auth_error, AuthError::UserNotFound));
    }

    #[test]
    fn test_sqlx_error_conversion_other() {
        let dummy_err = SqlxError::ColumnNotFound("some_column".to_string());
        let auth_error: AuthError = dummy_err.into();
        assert!(matches!(auth_error, AuthError::InvalidCredentials));
    }

    #[test]
    fn test_password_hash_error_conversion_from_argon2() {
        let err = Argon2Error::Password;
        let hash_error: PasswordHashError = err.into();
        assert!(matches!(hash_error, PasswordHashError::HashingError(_)));
    }

    #[test]
    fn test_validation_error_to_auth_error() {
        let err = ValidationError::new("invalid");
        let auth_error: AuthError = err.into();
        assert!(matches!(auth_error, AuthError::InvalidCredentials));
    }
}
