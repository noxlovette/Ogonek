
use crate::schema::AppState;
use crate::s3::{fetch, post};
use axum::routing::get;
use axum::Router;

pub fn s3_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(fetch::check_s3_connection).post(post::upload_file))
    .route(
        "/{key}",
        get(fetch::download_file)
    )
    .route("/stream/{key}", get(fetch::stream_file))
    .route(
        "/presign/{key}", get(fetch::get_presigned_url)
    )
}
