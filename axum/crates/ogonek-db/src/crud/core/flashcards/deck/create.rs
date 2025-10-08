use ogonek_types::{CardUpsert, DeckCreate};
use sqlx::PgPool;

use crate::{
    DbError,
    core::flashcards::{card, deck::read_deck},
};

/// Creates a new deck using fed data
pub(crate) async fn create(
    db: impl sqlx::Executor<'_, Database = sqlx::Postgres>,
    user_id: &str,
    create: DeckCreate,
) -> Result<String, DbError> {
    let id = sqlx::query_scalar!(
        r#"
        INSERT INTO decks (id, created_by, title, description, visibility, assignee)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id
        "#,
        nanoid::nanoid!(),
        user_id,
        create.title,
        create.description,
        create.visibility.unwrap_or_default().to_string(),
        create.assignee
    )
    .fetch_one(db)
    .await?;

    Ok(id)
}

/// Creates a copy of a deck
pub async fn duplicate(db: &PgPool, user_id: &str, deck_id: &str) -> Result<String, DbError> {
    let mut tx = db.begin().await?;

    let deck_to_copy = read_deck(db, deck_id, user_id).await?;
    let create_payload = DeckCreate {
        title: format!("{} (Copy)", deck_to_copy.title),
        description: deck_to_copy.description,
        visibility: Some(ogonek_types::Visibility::Private),
        assignee: None,
    };

    let cards = card::find_all(&mut *tx, deck_id).await?;

    let new_id = create(&mut *tx, user_id, create_payload).await?;
    let new_cards: Vec<CardUpsert> = cards
        .into_iter()
        .map(|card| CardUpsert {
            id: None,
            front: card.front,
            back: card.back,
            media_url: card.media_url,
        })
        .collect();

    card::batch_upsert(&mut *tx, &new_id, new_cards).await?;

    tx.commit().await?;

    Ok(new_id)
}

/// Creates a new deck with user defaults
pub async fn create_with_defaults(
    db: impl sqlx::Executor<'_, Database = sqlx::Postgres>,
    user_id: &str,
) -> Result<String, DbError> {
    let id = sqlx::query_scalar!(
        r#"
        INSERT INTO decks (id, created_by, title, description, visibility )
        VALUES ($1, $2, $3, $4, $5 )
        RETURNING id
        "#,
        nanoid::nanoid!(),
        user_id,
        "Default Deck",
        "tag1;tag2",
        "private"
    )
    .fetch_one(db)
    .await?;

    Ok(id)
}
