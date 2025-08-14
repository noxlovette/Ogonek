use crate::api::core::learn;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        learn::fetch_due_cards,
        learn::update_card_progress,
        learn::reset_deck_progress,
        learn::subscribe_to_deck,
        learn::unsubscribe_from_deck,
    ),
    components(schemas(
        crate::types::CardProgressWithFields,
        crate::types::UpdateCardProgress,
    ))
)]
pub struct LearnApi;
