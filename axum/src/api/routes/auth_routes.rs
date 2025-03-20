use crate::api::auth;
use crate::schema::AppState;
use axum::routing::{get, post};
use axum::Router;

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/signup", post(auth::signup))
        .route("/signin", post(auth::authorize))
        .route("/refresh", get(auth::refresh))
        .route("/invite", post(auth::generate_invite_link))
        .route("/bind", post(auth::bind_student_to_teacher))
}
