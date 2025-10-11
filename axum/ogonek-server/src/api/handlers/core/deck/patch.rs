use axum::{
    Json,
    extract::{Path, State},
};
use ogonek_db::{
    core::flashcards::{self, deck},
    tracking::{delete_seen, insert_as_unseen, log_activity},
};
use ogonek_notifications::NotificationType;
use ogonek_types::{ActionType, DeckWithCardsUpdate, ModelType};
use reqwest::StatusCode;

use crate::{AppError as APIError, AppState, Claims, api::DECK_TAG};

/// Updates a deck and its cards with assignment tracking
///
/// Modifies deck properties and handles assignee changes with notifications and activity logging.
#[utoipa::path(
    patch,
    path = "/{id}",
    params(
        ("id" = String, Path, description = "Deck ID")
    ),
    tag = DECK_TAG,
    request_body = DeckWithCardsUpdate,
    responses(
        (status = 204, description = "Deck updated successfully"),
        (status = 404, description = "Deck not found"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn update_deck(
    State(state): State<AppState>,
    claims: Claims,
    Path(id): Path<String>,
    Json(payload): Json<DeckWithCardsUpdate>,
) -> Result<StatusCode, APIError> {
    let current_assignee = deck::read_assignee(&state.db, &id, &claims.sub).await?;
    let new_assignee = payload.deck.assignee.clone();

    flashcards::deck::update(&state.db, &id, &claims.sub, payload).await?;

    if new_assignee != current_assignee {
        if let Some(old_user) = current_assignee {
            delete_seen(&state.db, &old_user, &id, ModelType::Deck).await?;
            flashcards::subscribe::unsubscribe(&state.db, &id, &old_user).await?;
            log_activity(
                &state.db,
                &claims.sub,
                &id,
                ModelType::Deck,
                ActionType::Delete,
                Some(&old_user),
            )
            .await?;
        }

        if let Some(new_user) = new_assignee {
            insert_as_unseen(&state.db, &new_user, &id, ModelType::Deck).await?;
            flashcards::subscribe::subscribe(&state.db, &id, &new_user).await?;
            log_activity(
                &state.db,
                &claims.sub,
                &id,
                ModelType::Deck,
                ActionType::Create,
                Some(&new_user),
            )
            .await?;

            let deck = flashcards::deck::read_deck(&state.db, &id, &claims.sub).await?;

            let _ = state
                .notification_service
                .notify_student(
                    &claims.sub,
                    &new_user,
                    NotificationType::DeckCreated {
                        deck_title: deck.title,
                        deck_id: deck.id,
                    },
                )
                .await;
        }
    } else if let Some(assignee) = current_assignee {
        // treat as update
        log_activity(
            &state.db,
            &claims.sub,
            &id,
            ModelType::Deck,
            ActionType::Update,
            Some(&assignee),
        )
        .await?;
    }
    Ok(StatusCode::NO_CONTENT)
}
