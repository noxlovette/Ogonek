use crate::api::core::{deck, learn};
use crate::schema::AppState;
use axum::Router;
use axum::routing::{get, patch, post};

pub fn deck_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(deck::fetch_deck_list).post(deck::create_deck))
        .route(
            "/{id}",
            get(deck::fetch_deck)
                .patch(deck::update_deck)
                .delete(deck::delete_deck)
                .post(learn::reset_deck_progress),
        )
        .route("/public", get(deck::fetch_deck_list_public))
        .route(
            "/learn/subscribe/{id}",
            post(learn::subscribe_to_deck).delete(learn::unsubscribe_from_deck),
        )
        .route("/learn/{id}", patch(learn::update_card_progress))
        .route("/learn", get(learn::fetch_due_cards))
}
