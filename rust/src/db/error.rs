use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("Database error")]
    Db,
    #[error("Not Found")]
    NotFound,
}

impl IntoResponse for DbError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            DbError::Db => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
            DbError::NotFound => (StatusCode::INTERNAL_SERVER_ERROR, "Not Found"),
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
