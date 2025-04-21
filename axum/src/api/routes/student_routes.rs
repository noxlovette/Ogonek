use crate::api::user;
use crate::schema::AppState;
use axum::routing::get;
use axum::Router;

pub fn student_routes() -> Router<AppState> {
    Router::new().route("/", get(user::list_students)).route(
        "/{id}",
        get(user::fetch_student)
            .post(user::upsert_student)
            .delete(user::remove_student)
            .patch(user::update_student),
    )
}
