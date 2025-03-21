use crate::api::file;
use crate::schema::AppState;
use axum::routing::{delete, get, post};
use axum::Router;

pub fn file_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(file::list_files))
        .route("/{file_id}", get(file::fetch_file).patch(file::update_file).delete(file::delete_file))
        .route("/folder", post(file::create_folder))
        .route("/folder/{folder_id}", delete(file::delete_folder))
}
