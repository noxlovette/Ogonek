use crate::api::file;
use crate::schema::AppState;
use axum::routing::{get, post};
use axum::Router;

pub fn file_routes() -> Router<AppState> {
    Router::new()
        .route("/upload", post(file::upload_handler))
        .route("/download/{filename}", get(file::download_handler))
}
