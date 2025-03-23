use crate::api::task;
use crate::schema::AppState;
use axum::routing::get;
use axum::Router;

pub fn task_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(task::list_tasks).post(task::create_task))
        .route("/recent", get(task::fetch_recent_tasks))
        .route(
            "/t/{id}",
            get(task::fetch_task)
                .patch(task::update_task)
                .delete(task::delete_task),
        )
}
