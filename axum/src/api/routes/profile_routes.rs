use crate::api::user;
use crate::schema::AppState;
use axum::routing::get;
use axum::Router;

pub fn profile_routes() -> Router<AppState> {
    Router::new().route("/", get(user::fetch_profile).patch(user::upsert_profile))
}
