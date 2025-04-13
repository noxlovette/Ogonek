use crate::s3::fetch;
use crate::schema::AppState;
use axum::routing::get;
use axum::Router;

pub fn s3_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(fetch::check_s3_connection))
        .route("/presign/{encoded_key}", get(fetch::get_presigned_url))
}
