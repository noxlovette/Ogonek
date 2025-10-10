use crate::{
    AppState,
    api::account::{self, resend_verification},
};
use axum::{Router, routing::post};

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/signup", post(account::signup))
        .route("/signin", post(account::signin))
        .route("/refresh", post(account::refresh))
        .route("/invite", post(account::generate_invite_link))
        .route("/bind", post(account::bind_student_to_teacher))
        .route("/confirm_email", post(account::confirm_email))
        .route("/resend", post(resend_verification))
}
