use crate::db::error::DbError;
use crate::models::{CardProgress, CardProgressWithFields, UpdateCardProgress};
use sqlx::PgPool;

pub async fn fetch_due(db: &PgPool, user_id: &str) -> Result<Vec<CardProgressWithFields>, DbError> {
    let due = sqlx::query_as!(
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
        user_id,
    )
    .fetch_all(db)
    .await?;

    Ok(due)
}

pub async fn find_by_id(
    db: &PgPool,
    progress_id: &str,
    user_id: &str,
) -> Result<CardProgress, DbError> {
    let progress = sqlx::query_as!(
        CardProgress,
        r#"
        SELECT cp.* FROM card_progress cp
        WHERE cp.user_id = $1 AND cp.id = $2
        "#,
        user_id,
        progress_id
    )
    .fetch_one(db)
    .await?;

    Ok(progress)
}

pub async fn update(
    db: &PgPool,
    card_id: &str,
    user_id: &str,
    update: UpdateCardProgress,
) -> Result<(), DbError> {
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
        update.review_count,
        update.ease_factor,
        update.interval,
        update.last_reviewed,
        update.due_date,
        user_id,
        card_id
    )
    .execute(db)
    .await?;

    Ok(())
}

pub async fn reset(db: &PgPool, deck_id: &str, user_id: &str) -> Result<(), DbError> {
    let mut tx = db.begin().await?;

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
        user_id
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(())
}
