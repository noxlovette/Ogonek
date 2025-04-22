use crate::db::error::DbError;
use crate::models::{
    CreationId, Deck, DeckCreate, DeckFilterParams, DeckSmall, DeckWithCardsUpdate,
};
use sqlx::PgPool;

pub async fn find_all(
    db: &PgPool,
    user_id: &str,
    params: &DeckFilterParams,
) -> Result<Vec<Deck>, DbError> {
    let mut query_builder = sqlx::QueryBuilder::new(
        "SELECT id, name, description, visibility, assignee, created_by, created_at
         FROM decks
         WHERE (created_by = ",
    );

    query_builder.push_bind(user_id);
    query_builder.push(" OR assignee = ");
    query_builder.push_bind(user_id);
    query_builder.push(")");

    if let Some(search) = &params.search {
        query_builder.push(" AND (name ILIKE ");
        query_builder.push_bind(format!("%{}%", search));
        query_builder.push(" OR description ILIKE ");
        query_builder.push_bind(format!("%{}%", search));
        query_builder.push(")");
    }

    if let Some(assignee) = &params.assignee {
        query_builder.push(" AND assignee = ");
        query_builder.push_bind(assignee);
    }

    query_builder.push(" ORDER BY created_at DESC");

    let decks = query_builder.build_query_as::<Deck>().fetch_all(db).await?;

    Ok(decks)
}

pub async fn find_by_id(db: &PgPool, deck_id: &str, user_id: &str) -> Result<Deck, DbError> {
    let deck = sqlx::query_as!(
        Deck,
        r#"
        SELECT id, name, description, visibility, assignee, created_by, created_at FROM decks
        WHERE id = $1 AND (
            created_by = $2
            OR assignee = $2
            OR visibility = 'public'
        )
        "#,
        deck_id,
        user_id
    )
    .fetch_one(db)
    .await?;

    Ok(deck)
}
pub async fn find_all_public(db: &PgPool) -> Result<Vec<DeckSmall>, DbError> {
    let decks = sqlx::query_as!(
        DeckSmall,
        r#"
 SELECT id, name, description
 FROM decks
 WHERE visibility = 'public'
 "#
    )
    .fetch_all(db)
    .await?;

    Ok(decks)
}

pub async fn create(db: &PgPool, user_id: &str, create: DeckCreate) -> Result<CreationId, DbError> {
    let visibility = if create.assignee.is_some() {
        create.visibility.unwrap_or("assigned".to_string())
    } else {
        create.visibility.unwrap_or("private".to_string())
    };
    let id = sqlx::query_as!(
        CreationId,
        r#"
        INSERT INTO decks (id, created_by, name, description, visibility, assignee)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id
        "#,
        nanoid::nanoid!(),
        user_id,
        create.name,
        create.description,
        visibility,
        create.assignee
    )
    .fetch_one(db)
    .await?;

    Ok(id)
}

pub async fn delete(db: &PgPool, deck_id: &str, user_id: &str) -> Result<(), DbError> {
    sqlx::query!(
        r#"
        DELETE FROM decks
        WHERE id = $1 AND created_by = $2
        "#,
        deck_id,
        user_id
    )
    .execute(db)
    .await?;

    Ok(())
}

pub async fn update(
    db: &PgPool,
    deck_id: &str,
    user_id: &str,
    update: DeckWithCardsUpdate,
) -> Result<(), DbError> {
    // Step 1: Update the deck
    sqlx::query!(
        "UPDATE decks
         SET
            name = COALESCE($1, name),
            description = COALESCE($2, description),
            visibility = COALESCE($3, visibility),
            assignee = COALESCE($4, assignee)
         WHERE id = $5 AND created_by = $6",
        update.deck.name,
        update.deck.description,
        update.deck.visibility,
        update.deck.assignee,
        deck_id,
        user_id
    )
    .execute(db)
    .await?;

    let mut tx = db.begin().await?;

    sqlx::query!(
        r#"
        DELETE FROM cards
        WHERE deck_id = $1 AND id NOT IN (
            SELECT UNNEST($2::text[])
        )
        "#,
        deck_id,
        &update
            .cards
            .iter()
            .filter_map(|i| i.id.clone())
            .collect::<Vec<String>>()
    )
    .execute(&mut *tx)
    .await?;

    // Step 3: Upsert (Insert or Update) Cards
    for card in update.cards {
        let card_id = card.id.clone().unwrap_or_else(|| nanoid::nanoid!());
        sqlx::query!(
            r#"
        INSERT INTO cards (id, deck_id, front, back, media_url)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (id) DO UPDATE
        SET
            front = EXCLUDED.front,
            back = EXCLUDED.back,
            deck_id = EXCLUDED.deck_id,
            media_url = EXCLUDED.media_url
        RETURNING *
        "#,
            card_id,
            deck_id,
            card.front,
            card.back,
            card.media_url,
        )
        .fetch_one(&mut *tx)
        .await?;
    }

    tx.commit().await?;

    Ok(())
}
pub async fn count(db: &PgPool, user_id: &str) -> Result<i64, DbError> {
    let count = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM decks WHERE
            (created_by = $1 OR assignee = $1)
            ",
        user_id
    )
    .fetch_one(db)
    .await?;

    Ok(count.unwrap_or(0))
}
