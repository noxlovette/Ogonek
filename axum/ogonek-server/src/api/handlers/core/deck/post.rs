use axum::{
    Json,
    extract::{Path, State},
};
use ogonek_db::{
    core::flashcards::{self},
    tracking::log_activity,
};
use ogonek_types::{ActionType, ModelType};

use crate::{AppError as APIError, AppState, Claims, api::DECK_TAG};

/// Creates a new Deck using user defaults
#[utoipa::path(
    post,
    tag = DECK_TAG,
    path = "",
    responses(
        (status = 200, description = "Deck created successfully", body = String),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn create_deck(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<String>, APIError> {
    let id = flashcards::deck::create_with_defaults(&state.db, &claims.sub).await?;

    log_activity(
        &state.db,
        &claims.sub,
        &id,
        ModelType::Deck,
        ActionType::Create,
        None,
    )
    .await?;

    Ok(Json(id))
}

/// Duplicates a deck, returns new id
#[utoipa::path(
    post,
    tag = DECK_TAG,
    path = "/{id}/duplicate",
    params(
        ("id" = String, Path, description = "Deck ID")
    ),
    responses(
        (status = 200, description = "Deck duplicated successfully", body = String),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn duplicate_deck(
    State(state): State<AppState>,
    claims: Claims,
    Path(id): Path<String>,
) -> Result<Json<String>, APIError> {
    let new_id = flashcards::deck::duplicate(&state.db, &claims.sub, &id).await?;

    log_activity(
        &state.db,
        &claims.sub,
        &new_id,
        ModelType::Deck,
        ActionType::Create,
        None,
    )
    .await?;

    Ok(Json(new_id))
}
