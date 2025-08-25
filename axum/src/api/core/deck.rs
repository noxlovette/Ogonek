use crate::api::error::APIError;
use crate::auth::Claims;
use crate::db::crud::core::flashcards::{self, deck};
use crate::db::crud::tracking::{delete_seen, insert_as_unseen, log_activity, mark_as_seen};
use crate::notifications::messages::NotificationType;
use crate::schema::AppState;
use crate::types::{
    ActionType, DeckPublic, DeckSmall, DeckWithCards, DeckWithCardsUpdate, ModelType,
    PaginatedDecks, PaginatedResponse, PaginationParams,
};
use axum::extract::{Json, Path, Query, State};
use axum::http::StatusCode;

use crate::api::DECK_TAG;
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
    path = "{id}/duplicate",
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

/// One deck
#[utoipa::path(
    get,
    tag = DECK_TAG,
    path = "/{id}",
    params(
        ("id" = String, Path, description = "Deck ID")
    ),
    responses(
        (status = 200, description = "Deck retrieved", body = DeckWithCards),
        (status = 404, description = "Deck not found"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn fetch_deck(
    State(state): State<AppState>,
    claims: Claims,
    Path(id): Path<String>,
) -> Result<Json<DeckWithCards>, APIError> {
    let deck_with_cards =
        flashcards::deck::get_deck_with_cards(&state.db, &id, &claims.sub).await?;

    mark_as_seen(&state.db, &claims.sub, &id, ModelType::Deck).await?;

    Ok(Json(deck_with_cards))
}

/// Decks the user has access to
#[utoipa::path(
    get,
    tag = DECK_TAG,
    path = "",
    params(
        ("page" = Option<u32>, Query, description = "Page number"),
        ("per_page" = Option<u32>, Query, description = "Items per page"),
        ("search" = Option<String>, Query, description = "Search term"),
        ("assignee" = Option<String>, Query, description = "Filter by assignee")
    ),
    responses(
        (status = 200, description = "User decks retrieved", body = PaginatedDecks),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_decks(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
    claims: Claims,
) -> Result<Json<PaginatedResponse<DeckSmall>>, APIError> {
    let decks = flashcards::deck::find_all(&state.db, &claims.sub, &params).await?;

    Ok(Json(PaginatedResponse {
        data: decks,
        page: params.page(),
        per_page: params.limit(),
    }))
}

/// Only public decks
#[utoipa::path(
    get,
    tag = DECK_TAG,
    path = "/public",
    responses(
        (status = 200, description = "Public decks retrieved", body=Vec<DeckPublic>),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_decks_public(
    State(state): State<AppState>,
) -> Result<Json<Vec<DeckPublic>>, APIError> {
    let decks = flashcards::deck::find_all_public(&state.db).await?;

    Ok(Json(decks))
}

/// Updates a deck
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
    let current_assignee = deck::find_assignee(&state.db, &id, &claims.sub).await?;
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

            let deck = flashcards::deck::get_deck(&state.db, &id, &claims.sub).await?;

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

/// Deletes a deck
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
    let assignee = deck::find_assignee(&state.db, &id, &claims.sub).await?;

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
