use crate::{
    api::{admin, middleware::require_elevated_role},
    schema::AppState,
};
use axum::{
    Router,
    middleware::from_fn,
    routing::{get, post, put},
};

pub fn admin_routes() -> Router<AppState> {
    Router::new()
        .nest("/users", user_routes())
        .nest("/content", content_routes())
        .layer(from_fn(require_elevated_role))
}

fn user_routes() -> Router<AppState> {
    Router::new().route("/", post(admin::user::create_user))
}

fn content_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            post(admin::content::create_content).get(admin::content::list_content),
        )
        .route(
            "/{id}",
            get(admin::content::fetch_content)
                .delete(admin::content::delete_content)
                .patch(admin::content::update_content),
        )
        .route(
            "/{id}/publish",
            put(admin::content::publish_content).delete(admin::content::unpublish_content),
        )
}
