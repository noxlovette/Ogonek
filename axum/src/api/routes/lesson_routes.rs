use crate::api::core;
use crate::schema::AppState;
use axum::routing::get;
use axum::Router;

pub fn lesson_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(core::list_lessons).post(core::create_lesson))
        .route("/recent", get(core::fetch_recent_lessons))
        .route(
            "/l/{id}",
            get(core::fetch_lesson)
                .patch(core::update_lesson)
                .delete(core::delete_lesson),
        )
}
