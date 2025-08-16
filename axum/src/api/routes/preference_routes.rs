use crate::api::account::preferences;
use crate::schema::AppState;
use axum::Router;
use axum::routing::get;

pub fn preferences_routes() -> Router<AppState> {
    Router::new().route(
        "/",
        get(preferences::get_preferences).patch(preferences::update_preferences),
    )
}
