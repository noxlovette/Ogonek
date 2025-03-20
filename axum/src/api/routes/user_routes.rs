use crate::api::user;
use crate::schema::AppState;
use axum::routing::get;
use axum::Router;

pub fn user_routes() -> Router<AppState> {
    Router::new().route(
        "/",
        get(user::fetch_user)
            .patch(user::update_user)
            .delete(user::delete_user),
    )
}
