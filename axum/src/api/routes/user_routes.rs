use crate::api::account;
use crate::schema::AppState;
use axum::Router;
use axum::routing::get;

pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(account::fetch_me)
                .patch(account::update_user)
                .delete(account::delete_user),
        )
        .route("/inviter", get(account::fetch_inviter))
}
