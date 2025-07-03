use crate::api::error::APIError;
use crate::auth::Claims;
use crate::db::crud::core;
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

    core::seen::mark_as_seen(
        &state.db,
        &claims.sub,
        &deck_id,
        core::seen::ModelType::Deck,
    )
    .await?;

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
            core::seen::delete_seen(&state.db, &old_user, &deck_id, core::seen::ModelType::Deck)
                .await?;
        }

        if let Some(new_user) = new_assignee {
            core::seen::insert_as_unseen(
                &state.db,
                &new_user,
                &deck_id,
                core::seen::ModelType::Deck,
            )
            .await?;
        }
    }
    Ok(StatusCode::NO_CONTENT)
}

pub async fn delete_deck(
    State(state): State<AppState>,
    claims: Claims,
    Path(deck_id): Path<String>,
) -> Result<StatusCode, APIError> {
    words::deck::delete(&state.db, &deck_id, &claims.sub).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn subscribe_to_deck(
    State(state): State<AppState>,
    claims: Claims,
    Path(deck_id): Path<String>,
) -> Result<StatusCode, APIError> {
    words::subscribe::subscribe(&state.db, &deck_id, &claims.sub).await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn unsubscribe_from_deck(
    State(state): State<AppState>,
    claims: Claims,
    Path(deck_id): Path<String>,
) -> Result<StatusCode, APIError> {
    words::subscribe::unsubscribe(&state.db, &deck_id, &claims.sub).await?;

    Ok(StatusCode::NO_CONTENT)
}
