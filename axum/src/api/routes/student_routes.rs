use crate::api::student;
use crate::db::init::AppState;
use axum::routing::get;
use axum::Router;

pub fn student_routes() -> Router<AppState> {
    Router::new().route(
        "/",
        get(student::list_teacher_students)
            .post(student::upsert_student)
            .delete(student::remove_student),
    )
}
