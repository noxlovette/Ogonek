use crate::api::LEARN_TAG;
use crate::api::error::APIError;
use crate::auth::Claims;
use crate::db::crud::core::flashcards;
use crate::db::crud::tracking::log_activity;
use crate::types::{
    ActionType, CardProgressWithFields, ModelType, ReviewPayload, UpdateCardProgress,
};
use crate::schema::AppState;
use crate::tools::sm2::SM2Calculator;
use axum::extract::{Json, Path, State};
use axum::http::StatusCode;
use chrono::{Duration, Utc};

/// Subscribes the user to the deck
#[utoipa::path(
    post,
    path = "/subscribe/{id}",
    tag = LEARN_TAG,params(
        ("id" = String, Path, description = "Deck ID")
    ),
    responses(
        (status = 204, description = "Deck subscribed to successfully"),
        (status = 404, description = "Deck not found"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn subscribe_to_deck(
    State(state): State<AppState>,
    claims: Claims,
    Path(id): Path<String>,
) -> Result<StatusCode, APIError> {
    flashcards::subscribe::subscribe(&state.db, &id, &claims.sub).await?;
    log_activity(
        &state.db,
        &claims.sub,
        &id,
        ModelType::Deck,
        ActionType::Subscribe,
        None,
    )
    .await?;

    Ok(StatusCode::NO_CONTENT)
}

/// Unsubscribes the user from the deck
#[utoipa::path(
    delete,
    path = "/subscribe/{id}",
    tag = LEARN_TAG,params(
        ("id" = String, Path, description = "Deck ID")
    ),
    responses(
        (status = 204, description = "Deck subscribed to successfully"),
        (status = 404, description = "Deck not found"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn unsubscribe_from_deck(
    State(state): State<AppState>,
    claims: Claims,
    Path(id): Path<String>,
) -> Result<StatusCode, APIError> {
    flashcards::subscribe::unsubscribe(&state.db, &id, &claims.sub).await?;
    log_activity(
        &state.db,
        &claims.sub,
        &id,
        ModelType::Deck,
        ActionType::Unsubscribe,
        None,
    )
    .await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Returns the list of all cards due for review
#[utoipa::path(
    get,
    tag = LEARN_TAG,
    path = "",
    responses(
        (status = 200, description = "Due cards fetched successfully", body = Vec<CardProgressWithFields>),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn fetch_due_cards(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<Vec<CardProgressWithFields>>, APIError> {
    let due = flashcards::learn::fetch_due(&state.db, &claims.sub).await?;

    Ok(Json(due))
}

/// Updates the learn progress on a card
#[utoipa::path(
    put,
    path = "/{id}",
    tag = LEARN_TAG,params(
        ("id" = String, Path, description = "Card ID")
    ),
    request_body = ReviewPayload,
    responses(
        (status = 204, description = "Card progress updated successfully"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn update_card_progress(
    State(state): State<AppState>,
    claims: Claims,
    Path(id): Path<String>,
    Json(payload): Json<ReviewPayload>,
) -> Result<StatusCode, APIError> {
    let calculator = SM2Calculator::default();

    let current_progress = flashcards::learn::find_by_id(&state.db, &id, &claims.sub).await?;

    let (new_ease, new_interval, new_review_count) = calculator.calculate_next_review(
        payload.quality,
        current_progress.ease_factor,
        current_progress.interval,
        current_progress.review_count,
    );

    let update = UpdateCardProgress {
        review_count: new_review_count,
        ease_factor: new_ease,
        interval: new_interval,
        last_reviewed: Utc::now(),
        due_date: Utc::now() + Duration::days(new_interval.into()),
    };
    flashcards::learn::update(&state.db, &id, &claims.sub, update).await?;

    Ok(StatusCode::NO_CONTENT)
}
/// Resets the progress for a particular deck
#[utoipa::path(
    delete,
    path = "/{id}",
    tag = LEARN_TAG,params(
        ("id" = String, Path, description = "Deck ID")
    ),
    responses(
        (status = 204, description = "Card progress updated successfully", body = Vec<CardProgressWithFields>),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn reset_deck_progress(
    State(state): State<AppState>,
    claims: Claims,
    Path(deck_id): Path<String>,
) -> Result<StatusCode, APIError> {
    flashcards::learn::reset(&state.db, &deck_id, &claims.sub).await?;

    Ok(StatusCode::OK)
}
