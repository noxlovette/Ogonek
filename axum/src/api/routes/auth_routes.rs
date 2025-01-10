use crate::api::auth;
use crate::db::init::AppState;
use axum::routing::{get, post};
use axum::Router;

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/signup", post(auth::signup))
        .route("/signin", post(auth::authorize))
        .route("/refresh", get(auth::refresh))
}
