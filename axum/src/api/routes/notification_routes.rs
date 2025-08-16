use crate::api::notifications::{self, request_hw};
use crate::schema::AppState;
use axum::Router;
use axum::routing::post;

pub fn notification_routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(notifications::register_device_token))
        .route("/request", post(request_hw))
}
