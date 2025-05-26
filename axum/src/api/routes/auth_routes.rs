use crate::api::account;
use crate::schema::AppState;
use axum::routing::post;
use axum::Router;

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/signup", post(account::signup))
        .route("/signin", post(account::authorize))
        .route("/refresh", post(account::refresh))
        .route("/invite", post(account::generate_invite_link))
        .route("/bind", post(account::bind_student_to_teacher))
}
