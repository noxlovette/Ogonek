use crate::db::error::DbError;
use sqlx::PgPool;

pub async fn subscribe(db: &PgPool, deck_id: &str, user_id: &str) -> Result<(), DbError> {
    let mut tx = db.begin().await?;

    sqlx::query!(
        r#"
    INSERT INTO deck_subscriptions (deck_id, user_id)
    VALUES ($1, $2)
    ON CONFLICT (deck_id, user_id) DO NOTHING
    "#,
        deck_id,
        user_id
    )
    .execute(&mut *tx)
    .await?;

    let cards = sqlx::query!(
        r#"
    SELECT c.id FROM cards c
    LEFT JOIN card_progress cp
        ON cp.card_id = c.id
        AND cp.user_id = $1
    WHERE c.deck_id = $2
    AND cp.id IS NULL
    "#,
        user_id,
        deck_id
    )
    .fetch_all(&mut *tx)
    .await?;

    for card in cards {
        sqlx::query!(
            r#"
        INSERT INTO card_progress
            (id, user_id, card_id, review_count, due_date, ease_factor, interval)
        VALUES
            ($1, $2, $3, 0, CURRENT_TIMESTAMP, 2.5, 1)
        ON CONFLICT (user_id, card_id) DO NOTHING
        "#,
            nanoid::nanoid!(),
            user_id,
            card.id,
        )
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;

    Ok(())
}

pub async fn unsubscribe(db: &PgPool, deck_id: &str, user_id: &str) -> Result<(), DbError> {
    sqlx::query!(
        r#"
        DELETE FROM deck_subscriptions
        WHERE deck_id = $1 AND user_id = $2
        "#,
        deck_id,
        user_id
    )
    .execute(db)
    .await?;

    // Optionally, you can also clean up card progress
    sqlx::query!(
        r#"
    DELETE FROM card_progress cp
    USING cards c
    WHERE cp. card_id = c. id
    AND c.deck_id = $1
    AND cp.user_id = $2
    "#,
        deck_id,
        user_id
    )
    .execute(db)
    .await?;

    Ok(())
}
pub async fn check_subscription(
    db: &PgPool,
    deck_id: &str,
    user_id: &str,
) -> Result<bool, DbError> {
    let is_subscribed = sqlx::query!(
        r#"
        SELECT EXISTS(
            SELECT 1 FROM deck_subscriptions
            WHERE deck_id = $1 AND user_id = $2
        ) as "is_subscribed!"
        "#,
        deck_id,
        user_id
    )
    .fetch_one(db)
    .await?
    .is_subscribed;

    Ok(is_subscribed)
}
