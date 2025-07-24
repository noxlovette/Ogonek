use crate::api::files;
use crate::s3::fetch;
use crate::schema::AppState;
use axum::Router;
use axum::routing::{get, post};

pub fn s3_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(fetch::check_s3_connection))
        .route("/init", post(files::init_multipart_upload))
        .route("/complete", post(files::complete_multipart_upload))
        .route("/abort", post(files::abort_multipart_upload))
}
