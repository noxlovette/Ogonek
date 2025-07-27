use crate::api::account::*;
use crate::api::core::dashboard;
use crate::schema::AppState;
use axum::Router;
use axum::routing::get;

pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(fetch_me).patch(update_user).delete(delete_user))
        .route("/inviter", get(fetch_inviter))
        .route("/dashboard", get(dashboard::fetch_dashboard))
        .nest("/student", student_routes())
        .nest("/profile", profile_routes())
}

fn student_routes() -> Router<AppState> {
    Router::new().route("/", get(list_students)).route(
        "/{id}",
        get(fetch_student)
            .delete(remove_student)
            .patch(update_student)
            .post(upsert_student),
    )
}

fn profile_routes() -> Router<AppState> {
    Router::new().route("/", get(fetch_profile).patch(upsert_profile))
}
