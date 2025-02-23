use crate::api::{decks, learning};
use crate::db::init::AppState;
use axum::routing::{get, post, patch};
use axum::Router;

pub fn deck_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(decks::fetch_deck_list).post(decks::create_deck))
        .route(
            "/{deck_id}",
            get(decks::fetch_deck)
                .patch(decks::update_deck)
                .delete(decks::delete_deck)
                .post(learning::reset_deck_progress)
        )
        .route(
            "/learn/init/{deck_id}",
            post(learning::init_deck_learning)
        )
        .route(
            "/learn/{card_id}"
                    , patch(learning::update_card_progress)
                
        )
        .route(
            "/learn",
            get(learning::fetch_due_cards)
        )
}
