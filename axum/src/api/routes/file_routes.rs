use crate::api::files;
use crate::schema::AppState;
use crate::services::generate_report_zip;
use axum::Router;
use axum::routing::{delete, get, post};

pub fn file_routes() -> Router<AppState> {
    Router::new()
        .route("/init", post(files::init_multipart_upload))
        .route("/complete", post(files::complete_multipart_upload))
        .route("/abort", post(files::abort_multipart_upload))
        .route("/presigned/{encoded_key}", get(files::fetch_presigned_url))
        .route(
            "/presigned/batch/{task_id}",
            post(files::fetch_presigned_urls_batch),
        )
        .route("/{file_id}", delete(files::delete_file))
        .route("/html", post(generate_report_zip))
}
