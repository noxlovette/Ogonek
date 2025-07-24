use crate::db::error::DbError;
use crate::models::Card;
use sqlx::PgPool;

pub async fn find_all(db: &PgPool, deck_id: &str) -> Result<Vec<Card>, DbError> {
    let cards = sqlx::query_as!(
        Card,
        r#"
        SELECT * FROM cards
        WHERE deck_id = $1
        ORDER BY created_at DESC
        "#,
        deck_id
    )
    .fetch_all(db)
    .await?;

    Ok(cards)
}
