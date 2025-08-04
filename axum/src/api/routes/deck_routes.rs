use crate::api::core::{deck, learn};
use crate::schema::AppState;
use axum::Router;
use axum::routing::{get, post};

pub fn deck_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(deck::list_decks).post(deck::create_deck))
        .route(
            "/{id}",
            get(deck::fetch_deck)
                .patch(deck::update_deck)
                .delete(deck::delete_deck)
                .post(learn::reset_deck_progress),
        )
        .route("/{id}/duplicate", post(deck::duplicate_deck))
        .route("/public", get(deck::list_decks_public))
}
