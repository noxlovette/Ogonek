use crate::api::error::APIError;
use crate::auth::Claims;
use crate::db::crud::tracking::activity::log_activity;
use crate::db::crud::tracking::seen::{delete_seen, insert_as_unseen, mark_as_seen};
use crate::db::crud::tracking::{self, ActionType, ModelType};
use crate::db::crud::words::{self, deck};
use crate::models::{
    CreationId, DeckCreate, DeckFilterParams, DeckFull, DeckPublic, DeckSmall,
    DeckWithCardsAndSubscription, DeckWithCardsUpdate,
};
use crate::schema::AppState;
use axum::extract::Json;
use axum::extract::Path;
use axum::extract::Query;
use axum::extract::State;
use axum::http::StatusCode;

pub async fn create_deck(
    State(state): State<AppState>,
    claims: Claims,
    Json(payload): Json<DeckCreate>,
) -> Result<Json<CreationId>, APIError> {
    let id = words::deck::create(&state.db, &claims.sub, payload).await?;

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

pub async fn fetch_deck(
    State(state): State<AppState>,
    claims: Claims,
    Path(deck_id): Path<String>,
) -> Result<Json<Option<DeckWithCardsAndSubscription>>, APIError> {
    let deck = words::deck::find_by_id(&state.db, &deck_id, &claims.sub).await?;
    let is_subscribed =
        words::subscribe::check_subscription(&state.db, &deck_id, &claims.sub).await?;

    let cards = words::card::find_all(&state.db, &deck_id).await?;

    mark_as_seen(&state.db, &claims.sub, &deck_id, ModelType::Deck).await?;

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

pub async fn fetch_deck_list(
    State(state): State<AppState>,
    Query(params): Query<DeckFilterParams>,
    claims: Claims,
) -> Result<Json<Vec<DeckSmall>>, APIError> {
    let decks = words::deck::find_all(&state.db, &claims.sub, &params).await?;
    Ok(Json(decks))
}

pub async fn fetch_deck_list_public(
    State(state): State<AppState>,
) -> Result<Json<Vec<DeckPublic>>, APIError> {
    let decks = words::deck::find_all_public(&state.db).await?;

    Ok(Json(decks))
}

pub async fn update_deck(
    State(state): State<AppState>,
    claims: Claims,
    Path(deck_id): Path<String>,
    Json(payload): Json<DeckWithCardsUpdate>,
) -> Result<StatusCode, APIError> {
    let current_assignee = deck::find_assignee(&state.db, &deck_id, &claims.sub).await?;
    let new_assignee = payload.deck.assignee.clone();

    words::deck::update(&state.db, &deck_id, &claims.sub, payload).await?;

    if new_assignee != current_assignee {
        if let Some(old_user) = current_assignee {
            delete_seen(&state.db, &old_user, &deck_id, ModelType::Deck).await?;
            words::subscribe::unsubscribe(&state.db, &deck_id, &old_user).await?;
            log_activity(
                &state.db,
                &claims.sub,
                &deck_id,
                ModelType::Deck,
                ActionType::Delete,
                Some(&old_user),
            )
            .await?;
        }

        if let Some(new_user) = new_assignee {
            insert_as_unseen(&state.db, &new_user, &deck_id, ModelType::Deck).await?;
            words::subscribe::subscribe(&state.db, &deck_id, &new_user).await?;
            log_activity(
                &state.db,
                &claims.sub,
                &deck_id,
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
            &deck_id,
            ModelType::Deck,
            ActionType::Update,
            Some(&assignee),
        )
        .await?;
    }
    Ok(StatusCode::NO_CONTENT)
}

pub async fn delete_deck(
    State(state): State<AppState>,
    claims: Claims,
    Path(deck_id): Path<String>,
) -> Result<StatusCode, APIError> {
    let mut target_id: Option<String> = None;
    let assignee = deck::find_assignee(&state.db, &deck_id, &claims.sub).await?;

    words::deck::delete(&state.db, &deck_id, &claims.sub).await?;
    if let Some(user) = assignee {
        tracking::seen::delete_seen(&state.db, &user, &deck_id, ModelType::Deck).await?;
        target_id = Some(user);
    }

    // log deletion activity
    log_activity(
        &state.db,
        &claims.sub,
        &deck_id,
        ModelType::Deck,
        ActionType::Delete,
        target_id.as_deref(),
    )
    .await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn subscribe_to_deck(
    State(state): State<AppState>,
    claims: Claims,
    Path(deck_id): Path<String>,
) -> Result<StatusCode, APIError> {
    words::subscribe::subscribe(&state.db, &deck_id, &claims.sub).await?;
    log_activity(
        &state.db,
        &claims.sub,
        &deck_id,
        ModelType::Deck,
        ActionType::Subscribe,
        None,
    )
    .await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn unsubscribe_from_deck(
    State(state): State<AppState>,
    claims: Claims,
    Path(deck_id): Path<String>,
) -> Result<StatusCode, APIError> {
    words::subscribe::unsubscribe(&state.db, &deck_id, &claims.sub).await?;
    log_activity(
        &state.db,
        &claims.sub,
        &deck_id,
        ModelType::Deck,
        ActionType::Unsubscribe,
        None,
    )
    .await?;
    Ok(StatusCode::NO_CONTENT)
}
