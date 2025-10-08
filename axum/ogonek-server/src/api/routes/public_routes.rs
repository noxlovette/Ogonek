use crate::{
    AppState,
    api::{
        content,
        core::{fetch_task_public, toggle_task_public},
    },
};
use axum::{Router, routing::get};

pub fn content_routes() -> Router<AppState> {
    Router::new()
        .route("/content/{slug}", get(content::fetch_content_public))
        .route("/task/{id}", get(fetch_task_public).put(toggle_task_public))
}
