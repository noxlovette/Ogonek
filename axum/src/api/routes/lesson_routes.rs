use crate::api::lesson;
use crate::db::init::AppState;
use axum::routing::get;
use axum::Router;

pub fn lesson_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(lesson::list_lessons).post(lesson::create_lesson))
        .route("/recent", get(lesson::fetch_recent_lessons))
        .route(
            "/l/{id}",
            get(lesson::fetch_lesson)
                .patch(lesson::update_lesson)
                .delete(lesson::delete_lesson),
        )
}
