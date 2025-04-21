use crate::api::core;
use crate::schema::AppState;
use axum::routing::get;
use axum::Router;

pub fn task_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(core::list_tasks).post(core::create_task))
        .route("/recent", get(core::fetch_recent_tasks))
        .route(
            "/t/{id}",
            get(core::fetch_task)
                .patch(core::update_task)
                .delete(core::delete_task),
        )
}
