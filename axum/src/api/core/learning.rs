use crate::api::error::APIError;
use crate::auth::Claims;
use crate::db::crud::words;
use crate::models::{CardProgressWithFields, ReviewPayload, UpdateCardProgress};
use crate::schema::AppState;
use crate::tools::sm2::SM2Calculator;
use axum::extract::{Json, Path, State};
use axum::http::StatusCode;
use time::{Duration, OffsetDateTime};

pub async fn fetch_due_cards(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<Vec<CardProgressWithFields>>, APIError> {
    let due = words::learning::fetch_due(&state.db, &claims.sub).await?;

    Ok(Json(due))
}

pub async fn fetch_due_cards_number(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<Option<i64>>, APIError> {
    let due = words::learning::fetch_due_count(&state.db, &claims.sub).await?;

    Ok(Json(due))
}

pub async fn update_card_progress(
    State(state): State<AppState>,
    claims: Claims,
    Path(card_id): Path<String>,
    Json(payload): Json<ReviewPayload>,
) -> Result<StatusCode, APIError> {
    let calculator = SM2Calculator::default();

    let current_progress = words::learning::find_by_id(&state.db, &card_id, &claims.sub).await?;

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
        last_reviewed: OffsetDateTime::now_utc(),
        due_date: OffsetDateTime::now_utc() + Duration::days(new_interval.into()),
    };
    words::learning::update(&state.db, &card_id, &claims.sub, update).await?;

    Ok(StatusCode::OK)
}

pub async fn reset_deck_progress(
    State(state): State<AppState>,
    claims: Claims,
    Path(deck_id): Path<String>,
) -> Result<StatusCode, APIError> {
    words::learning::reset(&state.db, &deck_id, &claims.sub).await?;

    Ok(StatusCode::OK)
}
