use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

impl From<surrealdb::Error> for AuthError {
    fn from(error: surrealdb::Error) -> Self {
        eprintln!("{error}");
        Self::SignUpFail
    }
}

impl From<PasswordHashError> for AuthError {
    fn from(error: PasswordHashError) -> Self {
        eprintln!("{error}");
        Self::WrongCredentials
    }
}

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Wrong credentials")]
    WrongCredentials,
    #[error("Missing credentials")]
    MissingCredentials,
    #[error("Failed to Sign Up")]
    SignUpFail,
    #[error("Token creation error")]
    TokenCreation,
    #[error("Invalid token")]
    InvalidToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::SignUpFail => (StatusCode::BAD_REQUEST, "Failed to sign up"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

use argon2::password_hash::Error as Argon2Error;

#[derive(Debug, thiserror::Error)]
pub enum PasswordHashError {
    #[error("Failed to hash password: {0}")]
    HashingError(#[from] Argon2Error),
    #[error("Password verification failed after hashiong")]
    VerificationError,
}
