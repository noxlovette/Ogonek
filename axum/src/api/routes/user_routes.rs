use crate::api::account;
use crate::schema::AppState;
use axum::routing::get;
use axum::Router;

pub fn user_routes() -> Router<AppState> {
    Router::new().route(
        "/",
        get(account::fetch_user)
            .patch(account::update_user)
            .delete(account::delete_user),
    )
}
