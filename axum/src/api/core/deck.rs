use crate::api::error::APIError;
use crate::api::openapi::DECK_TAG;
use crate::auth::Claims;
use crate::db::crud::flashcards::{self, deck};
use crate::db::crud::tracking::activity::log_activity;
use crate::db::crud::tracking::seen::{delete_seen, insert_as_unseen, mark_as_seen};
use crate::db::crud::tracking::{self, ActionType, ModelType};
use crate::models::{
    CreationId, DeckFilterParams, DeckFull, DeckPublic, DeckSmall, DeckWithCardsAndSubscription,
    DeckWithCardsUpdate,
};
use crate::schema::AppState;
use axum::extract::{Json, Path, Query, State};
use axum::http::StatusCode;
/// Creates a new Deck using user defaults
#[utoipa::path(
    post,
    tag = DECK_TAG,
    path = "/deck",
    responses(
        (status = 201, description = "Deck created successfully", body = CreationId),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized")
    ),
    security(("api_key" = []))
)]
pub async fn create_deck(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<CreationId>, APIError> {
    let id = flashcards::deck::create_with_defaults(&state.db, &claims.sub).await?;

    log_activity(
        &state.db,
        &claims.sub,
        &id.id,
        ModelType::Deck,
        ActionType::Create,
        None,
    )
    .await?;

    Ok(Json(id))
}

/// One deck
#[utoipa::path(
    get,
    tag = DECK_TAG,
    path = "/deck/{id}",
    params(
        ("id" = String, Path, description = "Deck ID")
    ),
    responses(
        (status = 200, description = "Deck retrieved"),
        (status = 404, description = "Deck not found"),
        (status = 401, description = "Unauthorized")
    ),
    security(("api_key" = []))
)]
pub async fn fetch_deck(
    State(state): State<AppState>,
    claims: Claims,
    Path(id): Path<String>,
) -> Result<Json<Option<DeckWithCardsAndSubscription>>, APIError> {
    let deck = flashcards::deck::find_by_id(&state.db, &id, &claims.sub).await?;
    let is_subscribed =
        flashcards::subscribe::check_subscription(&state.db, &id, &claims.sub).await?;

    let cards = flashcards::card::find_all(&state.db, &id).await?;

    mark_as_seen(&state.db, &claims.sub, &id, ModelType::Deck).await?;

    Ok(Json(Some(DeckWithCardsAndSubscription {
        deck: DeckFull {
            id: deck.id,
            name: deck.name,
            description: deck.description,
            visibility: deck.visibility,
            is_subscribed: deck.is_subscribed,
            created_by: deck.created_by,
            assignee: deck.assignee,
            created_at: deck.created_at,
        },
        cards,
        is_subscribed,
    })))
}

/// Decks the user has access to
#[utoipa::path(
    get,
    tag = DECK_TAG,
    path = "/deck",
    responses(
        (status = 200, description = "User decks retrieved"),
        (status = 404, description = "Deck not found"),
        (status = 401, description = "Unauthorized")
    ),
    security(("api_key" = []))
)]
pub async fn fetch_deck_list(
    State(state): State<AppState>,
    Query(params): Query<DeckFilterParams>,
    claims: Claims,
) -> Result<Json<Vec<DeckSmall>>, APIError> {
    let decks = flashcards::deck::find_all(&state.db, &claims.sub, &params).await?;
    Ok(Json(decks))
}

/// Only public decks
#[utoipa::path(
    get,
    tag = DECK_TAG,
    path = "/deck/public",
    responses(
        (status = 200, description = "Public decks retrieved"),
        (status = 404, description = "Deck not found"),
        (status = 401, description = "Unauthorized")
    ),
    security(("api_key" = []))
)]
pub async fn fetch_deck_list_public(
    State(state): State<AppState>,
) -> Result<Json<Vec<DeckPublic>>, APIError> {
    let decks = flashcards::deck::find_all_public(&state.db).await?;

    Ok(Json(decks))
}

/// Updates a deck
#[utoipa::path(
    patch,
    tag = DECK_TAG,

    path = "/deck/{id}",
    params(
        ("id" = String, Path, description = "Deck ID")
    ),
    request_body = DeckWithCardsUpdate,
    responses(
        (status = 204, description = "Deck updated successfully"),
        (status = 404, description = "Deck not found"),
        (status = 401, description = "Unauthorized")
    ),
    security(("api_key" = []))
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
    tag=DECK_TAG,
    path = "/deck/{id}",
    params(
        ("id" = String, Path, description = "Deck ID")
    ),
    responses(
        (status = 204, description = "Deck deleted successfully"),
        (status = 404, description = "Deck not found"),
        (status = 401, description = "Unauthorized")
    ),
    security(("api_key" = []))
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
        tracking::seen::delete_seen(&state.db, &user, &id, ModelType::Deck).await?;
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
