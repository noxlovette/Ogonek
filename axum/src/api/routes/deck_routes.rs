use crate::api::core::{decks, learning};
use crate::schema::AppState;
use axum::Router;
use axum::routing::{get, patch, post};

pub fn deck_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(decks::fetch_deck_list).post(decks::create_deck))
        .route(
            "/{deck_id}",
            get(decks::fetch_deck)
                .patch(decks::update_deck)
                .delete(decks::delete_deck)
                .post(learning::reset_deck_progress),
        )
        .route("/public", get(decks::fetch_deck_list_public))
        .route(
            "/learn/subscribe/{deck_id}",
            post(decks::subscribe_to_deck).delete(decks::unsubscribe_from_deck),
        )
        .route("/learn/{card_id}", patch(learning::update_card_progress))
        .route("/learn", get(learning::fetch_due_cards))
}
