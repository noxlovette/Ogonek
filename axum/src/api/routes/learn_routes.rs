use crate::api::core::learn;
use crate::schema::AppState;
use axum::Router;
use axum::routing::{get, patch, post};

pub fn learn_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/learn/subscribe/{id}",
            post(learn::subscribe_to_deck).delete(learn::unsubscribe_from_deck),
        )
        .route("/learn/{id}", patch(learn::update_card_progress))
        .route("/learn", get(learn::fetch_due_cards))
}
