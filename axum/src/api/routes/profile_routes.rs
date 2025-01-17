use crate::api::profile;
use crate::db::init::AppState;
use axum::routing::get;
use axum::Router;

pub fn profile_routes() -> Router<AppState> {
    Router::new().route(
        "/",
        get(profile::fetch_profile).patch(profile::upsert_profile),
    )
}
