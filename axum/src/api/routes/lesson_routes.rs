use crate::api::core;
use crate::schema::AppState;
use axum::Router;
use axum::routing::get;

pub fn lesson_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(core::list_lessons).post(core::create_lesson))
        .route(
            "/{id}",
            get(core::fetch_lesson)
                .patch(core::update_lesson)
                .delete(core::delete_lesson),
        )
}
