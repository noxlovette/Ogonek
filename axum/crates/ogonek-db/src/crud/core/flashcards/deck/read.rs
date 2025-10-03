use crate::{DbError, core::flashcards::card};

use ogonek_types::{DeckFull, DeckPaginationParams, DeckPublic, DeckSmall, DeckWithCards};
use sqlx::PgPool;
/// Builds a query based on page number, size, assignee ID
pub async fn read_all(
    db: &PgPool,
    user_id: &str,
    params: &DeckPaginationParams,
) -> Result<(Vec<DeckSmall>, i64), DbError> {
    let mut query_builder = sqlx::QueryBuilder::new(
        r#"SELECT
            d.id,
            d.title,
            d.description,
            d.visibility,
            d.created_at,
            d.updated_at,
            u.name as assignee_name,
            COALESCE(s.seen_at IS NOT NULL, TRUE) as seen,
            EXISTS (
                SELECT 1 FROM deck_subscriptions ds
                WHERE ds.deck_id = d.id AND ds.user_id = "#,
    );
    query_builder.push_bind(user_id);
    query_builder.push(
        r#") AS is_subscribed,
        d.card_count  
        FROM decks d
        LEFT JOIN "user" u ON d.assignee = u.id
        LEFT JOIN seen_status s ON s.model_id = d.id AND s.user_id = "#,
    );
    query_builder.push_bind(user_id);
    query_builder.push(" AND s.model_type = ");
    query_builder.push_bind("deck");

    // Base WHERE clause - user must be creator or assignee
    query_builder.push(" WHERE (d.created_by = ");
    query_builder.push_bind(user_id);
    query_builder.push(" OR d.assignee = ");
    query_builder.push_bind(user_id);
    query_builder.push(")");

    // Search filter
    if let Some(search) = &params.search {
        query_builder.push(" AND (d.title ILIKE ");
        query_builder.push_bind(format!("%{search}%"));
        query_builder.push(" OR d.description ILIKE ");
        query_builder.push_bind(format!("%{search}%"));
        query_builder.push(")");
    }

    // Assignee filter
    if let Some(assignee) = &params.assignee {
        query_builder.push(" AND d.assignee = ");
        query_builder.push_bind(assignee);
    }

    // Visibility filter (deck-specific)
    if let Some(visibility) = &params.visibility {
        query_builder.push(" AND d.visibility = ");
        query_builder.push_bind(visibility);
    }

    // Subscribed only filter (deck-specific)
    if let Some(true) = params.subscribed_only {
        query_builder.push(" AND EXISTS (");
        query_builder.push("SELECT 1 FROM deck_subscriptions ds ");
        query_builder.push("WHERE ds.deck_id = d.id AND ds.user_id = ");
        query_builder.push_bind(user_id);
        query_builder.push(")");
    }

    // Dynamic ordering
    query_builder.push(" ORDER BY ");
    query_builder.push(params.sort_by.to_deck_column());
    query_builder.push(" ");
    query_builder.push(params.sort_order.to_sql());

    // Pagination
    query_builder.push(" LIMIT ");
    query_builder.push_bind(params.limit());
    query_builder.push(" OFFSET ");
    query_builder.push_bind(params.offset());

    let decks = query_builder
        .build_query_as::<DeckSmall>()
        .fetch_all(db)
        .await?;
    // Build count query with same filters
    let mut count_query =
        sqlx::QueryBuilder::new(r#"SELECT COUNT(*) FROM decks l WHERE (l.assignee = "#);
    count_query.push_bind(user_id);
    count_query.push(" OR l.created_by = ");
    count_query.push_bind(user_id);
    count_query.push(")");

    if let Some(search) = &params.search {
        count_query.push(" AND (l.title ILIKE ");
        count_query.push_bind(format!("%{search}%"));
        count_query.push(" OR l.topic ILIKE ");
        count_query.push_bind(format!("%{search}%"));
        count_query.push(" OR l.markdown ILIKE ");
        count_query.push_bind(format!("%{search}%"));
        count_query.push(")");
    }

    if let Some(assignee) = &params.assignee {
        count_query.push(" AND l.assignee = ");
        count_query.push_bind(assignee);
    }

    let total: (i64,) = count_query.build_query_as().fetch_one(db).await?;
    Ok((decks, total.0))
}

pub async fn read_deck_with_cards(
    db: &PgPool,
    deck_id: &str,
    user_id: &str,
) -> Result<DeckWithCards, DbError> {
    let mut tx = db.begin().await?;

    let deck = read_deck(&mut *tx, deck_id, user_id).await?;
    let cards = card::find_all(&mut *tx, deck_id).await?;

    Ok(DeckWithCards { deck, cards })
}

/// Find a deck by id with card count and subscription status included
pub async fn read_deck(
    db: impl sqlx::Executor<'_, Database = sqlx::Postgres>,
    deck_id: &str,
    user_id: &str,
) -> Result<DeckFull, DbError> {
    let deck = sqlx::query_as!(
        DeckFull,
        r#"
        SELECT
            d.id,
            d.title,
            d.description,
            d.visibility,
            d.assignee,
            d.created_by,
            d.created_at,
            EXISTS (
                SELECT 1 FROM deck_subscriptions
                WHERE deck_id = d.id AND user_id = $2
            ) AS "is_subscribed!",
            (
                SELECT COUNT(*)::bigint FROM cards
                WHERE deck_id = d.id
            ) AS "card_count!"
        FROM decks d
        WHERE d.id = $1 AND (
            d.created_by = $2
            OR d.assignee = $2
            OR d.visibility = 'public'
            OR EXISTS (
                SELECT 1 FROM deck_subscriptions
                WHERE deck_id = $1 AND user_id = $2
            )
        )
        "#,
        deck_id,
        user_id
    )
    .fetch_one(db)
    .await?;
    Ok(deck)
}

pub async fn read_all_public(
    db: impl sqlx::Executor<'_, Database = sqlx::Postgres>,
) -> Result<Vec<DeckPublic>, DbError> {
    let decks = sqlx::query_as!(
        DeckPublic,
        r#"
 SELECT id, title, description
 FROM decks
 WHERE visibility = 'public'
 "#
    )
    .fetch_all(db)
    .await?;

    Ok(decks)
}
/// Finds the assignee for the deck
pub async fn read_assignee(
    db: impl sqlx::Executor<'_, Database = sqlx::Postgres>,
    lesson_id: &str,
    user_id: &str,
) -> Result<Option<String>, DbError> {
    let assignee = sqlx::query_scalar!(
        r#"
        SELECT assignee
        FROM decks
        WHERE id = $1
        AND (assignee = $2 OR created_by = $2)
        "#,
        lesson_id,
        user_id
    )
    .fetch_one(db) // in case lesson is not found
    .await?;

    Ok(assignee)
}

/// Count the overall number of decks
pub async fn count(
    db: impl sqlx::Executor<'_, Database = sqlx::Postgres>,
    user_id: &str,
) -> Result<i64, DbError> {
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
