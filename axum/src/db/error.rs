use crate::auth::error::PasswordHashError;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;
use serde_json::json;
use sqlx::error::Error as SqlxError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("Database error")]
    Db,
    #[error("Not Found")]
    NotFound,
    #[error("Transaction failed")]
    TransactionFailed,
    #[error("Already exists")]
    AlreadyExists,
}

impl IntoResponse for DbError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            DbError::Db => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
            DbError::NotFound => (StatusCode::INTERNAL_SERVER_ERROR, "Not Found"),
            DbError::TransactionFailed => (StatusCode::INTERNAL_SERVER_ERROR, "Transaction failed"),
            DbError::AlreadyExists => (StatusCode::INTERNAL_SERVER_ERROR, "Already exists"),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

impl From<SqlxError> for DbError {
    fn from(error: SqlxError) -> Self {
        eprintln!("Database error: {}", error);
        Self::Db
    }
}

impl From<PasswordHashError> for DbError {
    fn from(error: PasswordHashError) -> Self {
        eprintln!("{error}");
        Self::TransactionFailed
    }
}
