use axum::{
    Json,
    extract::{Path, State},
};
use ogonek_db::{
    core::flashcards::{self, deck},
    tracking::{delete_seen, log_activity},
};
use ogonek_types::{ActionType, ModelType};
use reqwest::StatusCode;

use crate::{AppError as APIError, AppState, Claims, api::DECK_TAG};
/// Deletes a single deck and associated data
///
/// Removes the deck and cleans up tracking data for assignees.
#[utoipa::path(
    delete,
    tag = DECK_TAG,
    path = "/{id}",
    params(
        ("id" = String, Path, description = "Deck ID")
    ),
    responses(
        (status = 204, description = "Deck deleted successfully"),
        (status = 404, description = "Deck not found"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn delete_deck(
    State(state): State<AppState>,
    claims: Claims,
    Path(id): Path<String>,
) -> Result<StatusCode, APIError> {
    let mut target_id: Option<String> = None;
    let assignee = deck::read_assignee(&state.db, &id, &claims.sub).await?;

    flashcards::deck::delete(&state.db, &id, &claims.sub).await?;
    if let Some(user) = assignee {
        delete_seen(&state.db, &user, &id, ModelType::Deck).await?;
        target_id = Some(user);
    }

    // log deletion activity
    log_activity(
        &state.db,
        &claims.sub,
        &id,
        ModelType::Deck,
        ActionType::Delete,
        target_id.as_deref(),
    )
    .await?;

    Ok(StatusCode::NO_CONTENT)
}

/// Deletes multiple decks in bulk
///
/// Removes multiple decks specified by their IDs for the authenticated user.
#[utoipa::path(
    delete,
    path = "/many",
    tag = DECK_TAG,
    request_body = Vec<String>,
    responses(
        (status = 204, description = "decks deleted successfully"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn delete_deck_many(
    State(state): State<AppState>,
    claims: Claims,
    Json(payload): Json<Vec<String>>,
) -> Result<StatusCode, APIError> {
    deck::delete_many(&state.db, payload, &claims.sub).await?;

    Ok(StatusCode::NO_CONTENT)
}
