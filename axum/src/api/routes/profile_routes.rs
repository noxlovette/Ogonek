use crate::api::account;
use crate::schema::AppState;
use axum::routing::get;
use axum::Router;

pub fn profile_routes() -> Router<AppState> {
    Router::new().route(
        "/",
        get(account::fetch_profile).patch(account::upsert_profile),
    )
}
