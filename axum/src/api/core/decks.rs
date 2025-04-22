use crate::api::error::APIError;
use crate::auth::jwt::Claims;
use crate::db::crud::learning;
use crate::models::{
    CreationId, Deck, DeckCreate, DeckFilterParams, DeckSmall, DeckWithCardsAndSubscription,
    DeckWithCardsUpdate,
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
    let id = learning::deck::create(&state.db, &claims.sub, payload).await?;

    Ok(Json(id))
}

pub async fn fetch_deck(
    State(state): State<AppState>,
    claims: Claims,
    Path(deck_id): Path<String>,
) -> Result<Json<Option<DeckWithCardsAndSubscription>>, APIError> {
    let deck = learning::deck::find_by_id(&state.db, &deck_id, &claims.sub).await?;
    let is_subscribed =
        learning::subscribe::check_subscription(&state.db, &deck_id, &claims.sub).await?;

    let cards = learning::card::find_all(&state.db, &deck_id).await?;

    Ok(Json(Some(DeckWithCardsAndSubscription {
        deck: Deck {
            id: deck.id,
            name: deck.name,
            description: deck.description,
            visibility: deck.visibility,
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
) -> Result<Json<Vec<Deck>>, APIError> {
    let decks = learning::deck::find_all(&state.db, &claims.sub, &params).await?;
    Ok(Json(decks))
}

pub async fn fetch_deck_list_public(
    State(state): State<AppState>,
) -> Result<Json<Vec<DeckSmall>>, APIError> {
    let decks = learning::deck::find_all_public(&state.db).await?;

    Ok(Json(decks))
}

pub async fn update_deck(
    State(state): State<AppState>,
    claims: Claims,
    Path(deck_id): Path<String>,
    Json(payload): Json<DeckWithCardsUpdate>,
) -> Result<StatusCode, APIError> {
    learning::deck::update(&state.db, &deck_id, &claims.sub, payload).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn delete_deck(
    State(state): State<AppState>,
    claims: Claims,
    Path(deck_id): Path<String>,
) -> Result<StatusCode, APIError> {
    learning::deck::delete(&state.db, &deck_id, &claims.sub).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn subscribe_to_deck(
    State(state): State<AppState>,
    claims: Claims,
    Path(deck_id): Path<String>,
) -> Result<StatusCode, APIError> {
    learning::subscribe::subscribe(&state.db, &claims.sub, &deck_id).await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn unsubscribe_from_deck(
    State(state): State<AppState>,
    claims: Claims,
    Path(deck_id): Path<String>,
) -> Result<StatusCode, APIError> {
    learning::subscribe::unsubscribe(&state.db, &claims.sub, &deck_id).await?;

    Ok(StatusCode::NO_CONTENT)
}
