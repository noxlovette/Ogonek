use crate::api::user;
use crate::db::init::AppState;
use axum::routing::get;
use axum::Router;

pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/self", get(user::fetch_user_self))
        .route(
            "/u/:id",
            get(user::fetch_user)
                .patch(user::update_user)
                .delete(user::delete_user),
        )
        .route("/all", get(user::list_users))
}
