use crate::api::multipart;
use crate::s3::fetch;
use crate::schema::AppState;
use axum::routing::{get, post};
use axum::Router;

pub fn s3_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(fetch::check_s3_connection))
        .route("/presign/{encoded_key}", get(fetch::get_presigned_url))
        .route("/multipart/init", post(multipart::init_multipart_upload))
        .route(
            "/multipart/complete",
            post(multipart::complete_multipart_upload),
        )
        .route("/multipart/abort", post(multipart::abort_multipart_upload))
}
