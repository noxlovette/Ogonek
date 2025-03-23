use crate::api::profile;
use crate::schema::AppState;
use axum::routing::get;
use axum::Router;

pub fn profile_routes() -> Router<AppState> {
    Router::new().route(
        "/",
        get(profile::fetch_profile).patch(profile::upsert_profile),
    )
}
