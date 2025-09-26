use crate::{AppState, api::content};
use axum::{Router, routing::get};

pub fn content_routes() -> Router<AppState> {
    Router::new().route("/{slug}", get(content::fetch_content_public))
}
