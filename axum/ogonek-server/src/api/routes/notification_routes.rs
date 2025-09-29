use crate::{
    AppState,
    api::notifications::{self, request_hw},
};
use axum::{Router, routing::post};

pub fn notification_routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(notifications::register_device_token))
        .route("/request", post(request_hw))
}
