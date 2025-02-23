use crate::api::{decks, learning};
use crate::db::init::AppState;
use axum::routing::{get, post};
use axum::Router;

pub fn deck_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(decks::fetch_deck_list).post(decks::create_deck))
        .route(
            "/{deck_id}",
            get(decks::fetch_deck)
                .patch(decks::update_deck)
                .delete(decks::delete_deck),
        )
        .route(
            "/learn/{card_id}",
                post(learning::create_card_progress)
                    .patch(learning::update_card_progress)
                
        )
        .route(
            "/learn",
            get(learning::fetch_due_cards)
        )
}
