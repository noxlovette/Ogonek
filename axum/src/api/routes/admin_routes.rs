use crate::{
    api::{admin, middleware::require_elevated_role},
    schema::AppState,
};
use axum::{Router, middleware::from_fn, routing::post};

pub fn admin_routes() -> Router<AppState> {
    Router::new()
        .nest("/users", user_routes())
        .layer(from_fn(require_elevated_role))
}

fn user_routes() -> Router<AppState> {
    Router::new().route("/", post(admin::user::create_user))
}
