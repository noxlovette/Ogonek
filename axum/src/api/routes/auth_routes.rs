use crate::api::account;
use crate::schema::AppState;
use axum::Router;
use axum::routing::post;

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/signup", post(account::signup))
        .route("/signin", post(account::signin))
        .route("/refresh", post(account::refresh))
        .route("/invite", post(account::generate_invite_link))
        .route("/bind", post(account::bind_student_to_teacher))
}
