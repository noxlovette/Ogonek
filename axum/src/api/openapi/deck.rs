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
    ),
    components(schemas(
        crate::types::DeckSmall,
        crate::types::DeckFull,
        crate::types::DeckUpdate,
        crate::types::DeckPublic,
    ))
)]
pub struct DeckApi;
