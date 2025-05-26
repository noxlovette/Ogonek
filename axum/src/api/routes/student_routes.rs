use crate::api::account;
use crate::schema::AppState;
use axum::routing::get;
use axum::Router;

pub fn student_routes() -> Router<AppState> {
    Router::new().route("/", get(account::list_students)).route(
        "/{id}",
        get(account::fetch_student)
            .post(account::upsert_student)
            .delete(account::remove_student)
            .patch(account::update_student),
    )
}
