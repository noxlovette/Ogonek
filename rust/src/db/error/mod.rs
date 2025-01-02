use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;
use thiserror::Error;
use crate::auth::AuthError;
use serde_json::json;

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)] Auth(#[from] AuthError),
    #[error(transparent)] Db(#[from] DbError), // Assuming you have a DbError type defined
}

#[derive(Error, Debug)]
pub enum DbError {
    #[error("database error")]
    Db,
    #[error("authentication error: {0}")] Auth(#[from] AuthError),
}

impl IntoResponse for DbError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            DbError::Db => (StatusCode::INTERNAL_SERVER_ERROR, "database error"),
            DbError::Auth(auth_err) => {
                match auth_err {
                    AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
                    AuthError::MissingCredentials =>
                        (StatusCode::BAD_REQUEST, "Missing credentials"),
                    AuthError::TokenCreation =>
                        (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
                    AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
                }
            }
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

impl From<surrealdb::Error> for DbError {
    fn from(error: surrealdb::Error) -> Self {
        eprintln!("{error}");
        Self::Db
    }
}
