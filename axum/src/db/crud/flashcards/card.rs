use crate::db::error::DbError;
use crate::models::{Card, CardUpsert};

pub async fn find_all(
    executor: impl sqlx::Executor<'_, Database = sqlx::Postgres>,
    deck_id: &str,
) -> Result<Vec<Card>, DbError> {
    let cards = sqlx::query_as!(
        Card,
        r#"
        SELECT * FROM cards
        WHERE deck_id = $1
        ORDER BY created_at DESC
        "#,
        deck_id
    )
    .fetch_all(executor)
    .await?;

    Ok(cards)
}

pub async fn batch_upsert(
    executor: impl sqlx::Executor<'_, Database = sqlx::Postgres>,
    deck_id: &str,
    cards: Vec<CardUpsert>,
) -> Result<(), DbError> {
    if cards.is_empty() {
        return Ok(());
    }

    let mut card_ids = Vec::with_capacity(cards.len());
    let mut fronts = Vec::with_capacity(cards.len());
    let mut backs = Vec::with_capacity(cards.len());
    let mut media_urls = Vec::with_capacity(cards.len());

    for card in cards {
        card_ids.push(card.id.clone().unwrap_or_else(|| nanoid::nanoid!()));
        fronts.push(card.front);
        backs.push(card.back);
        media_urls.push(card.media_url);
    }

    sqlx::query!(
        r#"
        INSERT INTO cards (id, deck_id, front, back, media_url)
        SELECT 
            UNNEST($1::text[]),
            $2,
            UNNEST($3::text[]),
            UNNEST($4::text[]),
            UNNEST($5::text[])
        ON CONFLICT (id) DO UPDATE SET
            front = EXCLUDED.front,
            back = EXCLUDED.back,
            deck_id = EXCLUDED.deck_id,
            media_url = EXCLUDED.media_url
        "#,
        &card_ids,
        deck_id,
        &fronts,
        &backs,
        &media_urls as &[Option<String>]
    )
    .execute(executor)
    .await?;

    Ok(())
}
