use crate::{
    api::notifications::{self, request_hw},
    schema::AppState,
};
use axum::{Router, routing::post};

pub fn notification_routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(notifications::register_device_token))
        .route("/request", post(request_hw))
}
