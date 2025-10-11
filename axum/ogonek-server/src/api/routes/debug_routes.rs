use axum::{Router, routing::post};

use crate::{AppState, api::send_confirm_email};

pub fn debug_routes() -> Router<AppState> {
    Router::new().route("/welcome", post(send_confirm_email))
}
