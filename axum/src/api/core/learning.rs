use crate::api::error::APIError;
use crate::auth::jwt::Claims;
use crate::models::cards_decks::{CardProgress, CardProgressWithFields, ReviewPayload};
use crate::schema::AppState;
use crate::tools::sm2::SM2Calculator;
use axum::extract::{Json, Path, State};
use axum::http::StatusCode;
use time::{Duration, OffsetDateTime};

pub async fn fetch_due_cards(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<Vec<CardProgressWithFields>>, APIError> {
    let progress_list = sqlx::query_as!(
        CardProgressWithFields,
        r#"
        SELECT
            cp.*,
            c.front,
            c.back,
            c.media_url
        FROM card_progress cp
        JOIN cards c ON c.id = cp.card_id
        WHERE cp.user_id = $1
            AND (cp.due_date <= CURRENT_TIMESTAMP OR cp.review_count = 0)
        ORDER BY cp.due_date ASC
        "#,
        claims.sub,
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(progress_list))
}

pub async fn update_card_progress(
    State(state): State<AppState>,
    claims: Claims,
    Path(card_id): Path<String>,
    Json(payload): Json<ReviewPayload>,
) -> Result<StatusCode, APIError> {
    let calculator = SM2Calculator::default();

    // Get current progress without transaction
    let current_progress = sqlx::query_as!(
        CardProgress,
        r#"
        SELECT cp.* FROM card_progress cp
        WHERE cp.user_id = $1 AND cp.id = $2
        "#,
        claims.sub,
        card_id
    )
    .fetch_one(&state.db)
    .await
    .map_err(|_| APIError::NotFound("Card progress not found".into()))?;

    // Calculate new values
    let (new_ease, new_interval, new_review_count) = calculator.calculate_next_review(
        payload.quality,
        current_progress.ease_factor,
        current_progress.interval,
        current_progress.review_count,
    );

    let now = OffsetDateTime::now_utc();
    let new_due_date = now + Duration::days(new_interval.into());

    // Simple update without returning data
    sqlx::query!(
        r#"
        UPDATE card_progress SET
            review_count = $1,
            ease_factor = $2,
            interval = $3,
            last_reviewed = $4,
            due_date = $5
        WHERE user_id = $6 AND id = $7
        "#,
        new_review_count,
        new_ease,
        new_interval,
        now,
        new_due_date,
        claims.sub,
        card_id
    )
    .execute(&state.db)
    .await?;

    Ok(StatusCode::OK)
}

pub async fn reset_deck_progress(
    State(state): State<AppState>,
    claims: Claims,
    Path(deck_id): Path<String>,
) -> Result<StatusCode, APIError> {
    let mut tx = state.db.begin().await?;

    let deck_exists = sqlx::query!(
        r#"
        SELECT 1 as "exists!: bool" FROM decks
        WHERE id = $1 AND (
            created_by = $2
            OR assignee = $2
            OR visibility = 'public'
        )
        "#,
        deck_id,
        claims.sub
    )
    .fetch_optional(&mut *tx)
    .await?
    .is_some();

    if !deck_exists {
        return Err(APIError::NotFound("Deck not found".into()));
    }

    sqlx::query!(
        r#"
        UPDATE card_progress cp
        SET
            review_count = 0,
            ease_factor = 2.5,
            interval = 1,
            last_reviewed = NULL,
            due_date = CURRENT_TIMESTAMP
        FROM cards c
        WHERE cp.card_id = c.id
        AND c.deck_id = $1
        AND cp.user_id = $2
        "#,
        deck_id,
        claims.sub
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(StatusCode::OK)
}
