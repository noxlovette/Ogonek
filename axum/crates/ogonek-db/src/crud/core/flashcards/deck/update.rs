use ogonek_types::DeckWithCardsUpdate;
use sqlx::PgPool;

use crate::{
    DbError,
    core::flashcards::card::{batch_upsert, delete_cards},
};

/// Updates a deck
pub async fn update(
    db: &PgPool,
    deck_id: &str,
    user_id: &str,
    update: DeckWithCardsUpdate,
) -> Result<(), DbError> {
    let mut tx = db.begin().await?;

    update_deck_solo(&mut *tx, deck_id, user_id, &update).await?;
    delete_cards(
        &mut *tx,
        deck_id,
        &update
            .cards
            .iter()
            .filter_map(|i| i.id.clone())
            .collect::<Vec<String>>(),
    )
    .await?;
    batch_upsert(&mut *tx, deck_id, update.cards).await?;

    tx.commit().await?;

    Ok(())
}

async fn update_deck_solo(
    executor: impl sqlx::Executor<'_, Database = sqlx::Postgres>,
    deck_id: &str,
    user_id: &str,
    update: &DeckWithCardsUpdate,
) -> Result<(), DbError> {
    sqlx::query!(
        "UPDATE decks
         SET
            title = COALESCE($1, title),
            description = COALESCE($2, description),
            visibility = COALESCE($3, visibility),
            assignee = COALESCE($4, assignee)
         WHERE id = $5 AND created_by = $6",
        update.deck.title,
        update.deck.description,
        update.deck.visibility.as_ref().map(|v| v.to_string()),
        update.deck.assignee,
        deck_id,
        user_id
    )
    .execute(executor)
    .await?;

    Ok(())
}
