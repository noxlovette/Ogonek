use crate::api::core::state::*;
use crate::schema::AppState;
use axum::Router;
use axum::routing::get;

pub fn state_routes() -> Router<AppState> {
    Router::new()
        .route("/dashboard", get(fetch_dashboard))
        .route("/badges", get(fetch_badges))
        .route("/context", get(fetch_context))
}
