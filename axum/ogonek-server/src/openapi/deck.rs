use crate::api::core::deck;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        deck::list_decks,
        deck::list_decks_public,
        deck::fetch_deck,
        deck::create_deck,
        deck::update_deck,
        deck::delete_deck,
        deck::duplicate_deck,
        deck::delete_deck_many,
    ),
    components(schemas(
        ogonek_types::DeckSmall,
        ogonek_types::DeckFull,
        ogonek_types::DeckUpdate,
        ogonek_types::DeckPublic,
    ))
)]
pub struct DeckApi;
