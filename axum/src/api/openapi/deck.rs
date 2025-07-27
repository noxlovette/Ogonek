use crate::api::core::deck;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        deck::fetch_deck_list,
        deck::fetch_deck_list_public,
        deck::fetch_deck,
        deck::create_deck,
        deck::update_deck,
        deck::delete_deck,
    ),
    components(schemas(
        crate::models::DeckSmall,
        crate::models::DeckFull,
        crate::models::DeckUpdate,
        crate::models::DeckPublic,
    ))
)]
pub struct DeckApi;
