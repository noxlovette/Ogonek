use crate::api::user;
use crate::schema::AppState;
use axum::routing::{get, post};
use axum::Router;

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/signup", post(user::signup))
        .route("/signin", post(user::authorize))
        .route("/refresh", get(user::refresh))
        .route("/invite", post(user::generate_invite_link))
        .route("/bind", post(user::bind_student_to_teacher))
}
