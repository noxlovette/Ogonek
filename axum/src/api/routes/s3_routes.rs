use crate::api::files;
use crate::s3::fetch;
use crate::schema::AppState;
use axum::Router;
use axum::routing::{delete, get, post};

pub fn s3_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(fetch::check_s3_connection))
        .route("/init", post(files::init_multipart_upload))
        .route("/complete", post(files::complete_multipart_upload))
        .route("/abort", post(files::abort_multipart_upload))
        .route("/presigned/{encoded_key}", get(files::fetch_presigned_url))
        .route(
            "/presigned/batch/{task_id}",
            post(files::fetch_presigned_urls_batch),
        )
        .route("/{file_id}", delete(files::delete_file))
}
