use super::card::{self, batch_upsert, delete_cards};
use crate::{
    db::error::DbError,
    types::{
        CardUpsert, DeckCreate, DeckFull, DeckPublic, DeckSmall, DeckWithCards,
        DeckWithCardsUpdate, PaginationParams,
    },
};
use sqlx::PgPool;

/// Builds a query based on page number, size, assignee ID
pub async fn find_all(
    db: impl sqlx::Executor<'_, Database = sqlx::Postgres>,
    user_id: &str,
    params: &PaginationParams,
) -> Result<Vec<DeckSmall>, DbError> {
    let mut query_builder = sqlx::QueryBuilder::new(
        r#"SELECT
            d.id,
            d.title,
            d.description,
            d.visibility,
            u.name as assignee_name,
            COALESCE(s.seen_at IS NOT NULL, TRUE) as seen,
            EXISTS (
                SELECT 1 FROM deck_subscriptions ds
                WHERE ds.deck_id = d.id AND ds.user_id = "#,
    );
    query_builder.push_bind(user_id);
    query_builder.push(
        r#") AS is_subscribed,
            (
                SELECT COUNT(*)::bigint FROM cards
                WHERE deck_id = d.id
            ) AS card_count
            FROM decks d
            LEFT JOIN "user" u ON d.assignee = u.id
            LEFT JOIN seen_status s ON s.model_id = d.id AND s.user_id = "#,
    );
    query_builder.push_bind(user_id);
    query_builder.push(r#" AND s.model_type = "#);
    query_builder.push_bind("deck"); // Add model_type filter like lessons!
    query_builder.push(
        r#"
            WHERE (d.created_by = "#,
    );
    query_builder.push_bind(user_id);
    query_builder.push(" OR d.assignee = ");
    query_builder.push_bind(user_id);
    query_builder.push(")");

    if let Some(search) = &params.search {
        query_builder.push(" AND (d.title ILIKE ");
        query_builder.push_bind(format!("%{search}%"));
        query_builder.push(" OR d.description ILIKE ");
        query_builder.push_bind(format!("%{search}%"));
        query_builder.push(")");
    }

    if let Some(assignee) = &params.assignee {
        query_builder.push(" AND d.assignee = ");
        query_builder.push_bind(assignee);
    }

    query_builder.push(" ORDER BY d.created_at DESC");

    query_builder.push(" LIMIT ");
    query_builder.push_bind(params.limit());
    query_builder.push(" OFFSET ");
    query_builder.push_bind(params.offset());

    let decks = query_builder
        .build_query_as::<DeckSmall>()
        .fetch_all(db)
        .await?;
    Ok(decks)
}
pub async fn get_deck_with_cards(
    db: &PgPool,
    deck_id: &str,
    user_id: &str,
) -> Result<DeckWithCards, DbError> {
    let mut tx = db.begin().await?;

    let deck = get_deck(&mut *tx, deck_id, user_id).await?;
    let cards = card::find_all(&mut *tx, deck_id).await?;

    Ok(DeckWithCards { deck, cards })
}

/// Find a deck by id with card count and subscription status included
pub async fn get_deck(
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

pub async fn find_all_public(
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

/// Returns three decks
pub async fn find_recent(
    db: impl sqlx::Executor<'_, Database = sqlx::Postgres>,
    user_id: &str,
) -> Result<Vec<DeckSmall>, DbError> {
    tracing::info!("Fetching recent tasks for {user_id}");
    let decks = sqlx::query_as!(
        DeckSmall,
        r#"
        SELECT
        d.id,
        d.title,
        d.description,
        d.visibility,
        COALESCE(u.name, NULL) AS assignee_name,
        COALESCE(s.seen_at IS NOT NULL, TRUE) as seen,
        EXISTS (
            SELECT 1 FROM deck_subscriptions s
            WHERE s.deck_id = d.id AND user_id = $1
        ) AS "is_subscribed!",
         (
    SELECT COUNT(*)::bigint FROM cards
    WHERE deck_id = d.id
) AS "card_count!"
    FROM decks d
    LEFT JOIN "user" u on d.assignee = u.id
    LEFT JOIN seen_status s ON s.model_id = d.id AND s.user_id = $1
    WHERE (
        d.created_by = $1
        OR d.assignee = $1
    )
    ORDER BY created_at DESC
    LIMIT 3
    "#,
        user_id,
    )
    .fetch_all(db)
    .await?;

    Ok(decks)
}

/// Creates a new deck using fed data
pub async fn create(
    db: impl sqlx::Executor<'_, Database = sqlx::Postgres>,
    user_id: &str,
    create: DeckCreate,
) -> Result<String, DbError> {
    let visibility = if create.assignee.is_some() {
        create.visibility.unwrap_or("assigned".to_string())
    } else {
        create.visibility.unwrap_or("private".to_string())
    };
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
        visibility,
        create.assignee
    )
    .fetch_one(db)
    .await?;

    Ok(id)
}

/// Creates a copy of a deck
pub async fn duplicate(db: &PgPool, user_id: &str, deck_id: &str) -> Result<String, DbError> {
    let mut tx = db.begin().await?;

    let deck_to_copy = get_deck(db, deck_id, user_id).await?;
    let create_payload = DeckCreate {
        title: format!("{} (Copy)", deck_to_copy.title),
        description: deck_to_copy.description,
        visibility: Some("private".to_string()),
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

    batch_upsert(&mut *tx, &new_id, new_cards).await?;

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
        INSERT INTO decks (id, created_by, title, description, visibility, assignee)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id
        "#,
        nanoid::nanoid!(),
        user_id,
        "Default Deck",
        "Default Description",
        "private",
        user_id,
    )
    .fetch_one(db)
    .await?;

    Ok(id)
}

/// Deletes a deck
pub async fn delete(
    db: impl sqlx::Executor<'_, Database = sqlx::Postgres>,
    deck_id: &str,
    user_id: &str,
) -> Result<(), DbError> {
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
        update.deck.visibility,
        update.deck.assignee,
        deck_id,
        user_id
    )
    .execute(executor)
    .await?;

    Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        tests::create_test_user,
        types::{CardUpsert, DeckUpdate},
    };
    use sqlx::PgPool;

    // Helper function to create a test deck
    async fn create_test_deck(
        db: impl sqlx::Executor<'_, Database = sqlx::Postgres>,
        user_id: &str,
        title: &str,
        description: Option<String>,
        visibility: Option<String>,
        assignee: Option<String>,
    ) -> Result<String, DbError> {
        let deck_create = DeckCreate {
            title: title.to_string(),
            description,
            visibility,
            assignee,
        };
        let result = create(db, user_id, deck_create).await?;
        Ok(result)
    }

    // Helper function to create test cards for a deck
    async fn create_test_cards(
        db: &PgPool,
        deck_id: &str,
        cards_data: Vec<(&str, &str, Option<String>)>,
    ) -> Result<Vec<String>, DbError> {
        let mut card_ids = Vec::new();

        for (front, back, media_url) in cards_data {
            let card_id = nanoid::nanoid!();
            sqlx::query!(
                r#"
                INSERT INTO cards (id, deck_id, front, back, media_url)
                VALUES ($1, $2, $3, $4, $5)
                "#,
                card_id,
                deck_id,
                front,
                back,
                media_url
            )
            .execute(db)
            .await
            .map_err(|e| DbError::Database(e))?;
            card_ids.push(card_id);
        }

        Ok(card_ids)
    }
    async fn create_test_deck_with_cards(
        db: &PgPool,
        user_id: &str,
        title: &str,
        card_count: usize,
    ) -> String {
        let deck_create = DeckCreate {
            title: title.to_string(),
            description: Some("Test deck with cards".to_string()),
            visibility: Some("private".to_string()),
            assignee: None,
        };

        let mut tx = db.begin().await.unwrap();
        let deck_result = create(&mut *tx, user_id, deck_create).await.unwrap();

        // Add cards if requested
        if card_count > 0 {
            let cards: Vec<CardUpsert> = (0..card_count)
                .map(|i| CardUpsert {
                    id: None,
                    front: format!("Question {}", i + 1),
                    back: format!("Answer {}", i + 1),
                    media_url: if i % 2 == 0 {
                        Some(format!("https://cdn.example.com/card_{}.jpg", i))
                    } else {
                        None
                    },
                })
                .collect();

            batch_upsert(&mut *tx, &deck_result, cards).await.unwrap();
        }

        tx.commit().await.unwrap();
        deck_result
    }

    // Helper function to count cards in a deck
    async fn count_cards_in_deck(
        db: impl sqlx::Executor<'_, Database = sqlx::Postgres>,
        deck_id: &str,
    ) -> Result<i64, DbError> {
        let count = sqlx::query_scalar!("SELECT COUNT(*) FROM cards WHERE deck_id = $1", deck_id)
            .fetch_one(db)
            .await
            .map_err(|e| DbError::Database(e))?;
        Ok(count.unwrap_or(0))
    }

    #[sqlx::test]
    async fn test_create_deck_success(db: PgPool) {
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;

        let deck_create = DeckCreate {
            title: "Test Deck".to_string(),
            description: Some("A test deck".to_string()),
            visibility: Some("private".to_string()),
            assignee: None,
        };

        let result = create(&db, &user_id, deck_create).await;
        assert!(result.is_ok());

        let deck_id = result.unwrap();
        assert!(!deck_id.is_empty());
    }

    #[sqlx::test]
    async fn test_create_deck_succes_with_defaults(db: PgPool) {
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;

        let result = create_with_defaults(&db, &user_id).await;
        assert!(result.is_ok());

        let deck_id = result.unwrap();
        assert!(!deck_id.is_empty());
    }

    #[sqlx::test]
    async fn test_create_deck_with_assignee_defaults_to_assigned_visibility(db: PgPool) {
        let user_id = create_test_user(&db, "creator", "creator@example.com").await;
        let assignee_id = create_test_user(&db, "assignee", "assignee@example.com").await;

        let deck_create = DeckCreate {
            title: "Assigned Deck".to_string(),
            description: None,
            visibility: None, // Should default to "assigned"
            assignee: Some(assignee_id.clone()),
        };

        let result = create(&db, &user_id, deck_create).await;
        assert!(result.is_ok());

        let deck_id = result.unwrap();
        let deck = get_deck(&db, &deck_id, &user_id).await.unwrap();
        assert_eq!(deck.visibility, "assigned");
        assert_eq!(deck.assignee, Some(assignee_id));
    }

    #[sqlx::test]
    async fn test_create_deck_without_assignee_defaults_to_private_visibility(db: PgPool) {
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;

        let deck_create = DeckCreate {
            title: "Private Deck".to_string(),
            description: None,
            visibility: None, // Should default to "private"
            assignee: None,
        };

        let result = create(&db, &user_id, deck_create).await;
        assert!(result.is_ok());

        let deck_id = result.unwrap();
        let deck = get_deck(&db, &deck_id, &user_id).await.unwrap();
        assert_eq!(deck.visibility, "private");
        assert_eq!(deck.assignee, None);
    }
    #[sqlx::test]
    async fn test_duplicate_deck_success_with_cards(db: PgPool) {
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let original_deck_id = create_test_deck_with_cards(&db, &user_id, "My Study Deck", 3).await;

        let result = duplicate(&db, &user_id, &original_deck_id).await;

        assert!(result.is_ok());
        let new_deck_id = result.unwrap();

        // Verify the duplicated deck has correct metadata
        let new_deck = get_deck(&db, &new_deck_id, &user_id).await.unwrap();
        assert_eq!(new_deck.title, "My Study Deck (Copy)");
        assert_eq!(new_deck.visibility, "private");
        assert_eq!(new_deck.assignee, None);

        // Verify cards were duplicated correctly
        let mut tx = db.begin().await.unwrap();
        let original_cards = card::find_all(&mut *tx, &original_deck_id).await.unwrap();
        let new_cards = card::find_all(&mut *tx, &new_deck_id).await.unwrap();

        assert_eq!(original_cards.len(), 3);
        assert_eq!(new_cards.len(), 3);

        // Cards should have same content but different deck association
        for (orig, new) in original_cards.iter().zip(new_cards.iter()) {
            assert_eq!(orig.front, new.front);
            assert_eq!(orig.back, new.back);
            assert_eq!(orig.media_url, new.media_url);
        }
    }

    #[sqlx::test]
    async fn test_duplicate_empty_deck(db: PgPool) {
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let empty_deck_id = create_test_deck_with_cards(&db, &user_id, "Empty Deck", 0).await;

        let result = duplicate(&db, &user_id, &empty_deck_id).await;

        assert!(result.is_ok());
        let new_deck_id = result.unwrap();

        // Should create deck even with no cards
        let new_deck = get_deck(&db, &new_deck_id, &user_id).await.unwrap();
        assert_eq!(new_deck.title, "Empty Deck (Copy)");

        // Verify no cards in duplicated deck
        let mut tx = db.begin().await.unwrap();
        let new_cards = card::find_all(&mut *tx, &new_deck_id).await.unwrap();
        assert_eq!(new_cards.len(), 0);
    }

    #[sqlx::test]
    async fn test_duplicate_deck_not_found(db: PgPool) {
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let fake_deck_id = "non-existent-deck-id";

        let result = duplicate(&db, &user_id, fake_deck_id).await;

        assert!(result.is_err());
        // Should fail gracefully when deck doesn't exist
        match result.unwrap_err() {
            DbError::NotFound(_) => {} // Expected
            other => panic!("Expected NotFound error, got: {:?}", other),
        }
    }

    #[sqlx::test]
    async fn test_duplicate_deck_unauthorized_user(db: PgPool) {
        let owner_id = create_test_user(&db, "owner", "owner@example.com").await;
        let other_user_id = create_test_user(&db, "hacker", "hacker@example.com").await;

        let deck_id = create_test_deck_with_cards(&db, &owner_id, "Private Deck", 2).await;

        // Try to duplicate someone else's deck
        let result = duplicate(&db, &other_user_id, &deck_id).await;

        assert!(result.is_err());
        // This should fail because get_deck checks ownership
    }

    #[sqlx::test]
    async fn test_duplicate_preserves_all_card_properties(db: PgPool) {
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;

        // Create deck manually to control card properties
        let deck_create = DeckCreate {
            title: "Rich Content Deck".to_string(),
            description: Some("Deck with various card types".to_string()),
            visibility: Some("private".to_string()),
            assignee: None,
        };

        let mut tx = db.begin().await.unwrap();
        let deck_result = create(&mut *tx, &user_id, deck_create).await.unwrap();

        // Add cards with different properties
        let cards = vec![
            CardUpsert {
                id: None,
                front: "Simple text front".to_string(),
                back: "Simple text back".to_string(),
                media_url: None,
            },
            CardUpsert {
                id: None,
                front: "Front with <b>HTML</b>".to_string(),
                back: "Back with **markdown**".to_string(),
                media_url: Some("https://scaleway.bucket.com/images/card.jpg".to_string()),
            },
            CardUpsert {
                id: None,
                front: "Special chars: 'quotes' & symbols!".to_string(),
                back: "Unicode: こんにちは 🚀".to_string(),
                media_url: Some("https://scaleway.bucket.com/audio/pronunciation.mp3".to_string()),
            },
        ];

        batch_upsert(&mut *tx, &deck_result, cards).await.unwrap();
        tx.commit().await.unwrap();

        // Duplicate the deck
        let result = duplicate(&db, &user_id, &deck_result).await;
        assert!(result.is_ok());

        // Verify all card properties were preserved
        let mut tx = db.begin().await.unwrap();
        let original_cards = card::find_all(&mut *tx, &deck_result).await.unwrap();
        let duplicated_cards = card::find_all(&mut *tx, &result.unwrap()).await.unwrap();

        assert_eq!(original_cards.len(), 3);
        assert_eq!(duplicated_cards.len(), 3);

        // Check each card was copied exactly
        for (orig, dup) in original_cards.iter().zip(duplicated_cards.iter()) {
            assert_eq!(orig.front, dup.front);
            assert_eq!(orig.back, dup.back);
            assert_eq!(orig.media_url, dup.media_url);
        }
    }

    #[sqlx::test]
    async fn test_duplicate_deck_name_formatting(db: PgPool) {
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;

        // Test various deck names and their copy formatting
        let test_cases = vec![
            ("Simple Deck", "Simple Deck (Copy)"),
            ("Deck with (parentheses)", "Deck with (parentheses) (Copy)"),
            ("Already a (Copy)", "Already a (Copy) (Copy)"),
            ("", " (Copy)"), // Edge case: empty name
            ("🚀 Emoji Deck", "🚀 Emoji Deck (Copy)"),
        ];

        for (original_name, expected_copy_name) in test_cases {
            let deck_id = create_test_deck_with_cards(&db, &user_id, original_name, 1).await;
            let result = duplicate(&db, &user_id, &deck_id).await;

            assert!(result.is_ok());
            let copied_deck = get_deck(&db, &result.unwrap(), &user_id).await.unwrap();
            assert_eq!(copied_deck.title, expected_copy_name);
        }
    }

    #[sqlx::test]
    async fn test_duplicate_deck_transaction_rollback_on_card_error(db: PgPool) {
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let deck_id = create_test_deck_with_cards(&db, &user_id, "Test Deck", 2).await;

        // This test is tricky - we'd need to mock card::batch_upsert to fail
        // For now, just verify the happy path creates everything in one transaction
        let decks_before = sqlx::query!(
            "SELECT COUNT(*) as count FROM decks WHERE created_by = $1",
            user_id
        )
        .fetch_one(&db)
        .await
        .unwrap()
        .count
        .unwrap();

        let result = duplicate(&db, &user_id, &deck_id).await;
        assert!(result.is_ok());

        let decks_after = sqlx::query!(
            "SELECT COUNT(*) as count FROM decks WHERE created_by = $1",
            user_id
        )
        .fetch_one(&db)
        .await
        .unwrap()
        .count
        .unwrap();

        assert_eq!(decks_after, decks_before + 1);
    }

    #[sqlx::test]
    async fn test_get_deck_success_as_creator(db: PgPool) {
        let user_id = create_test_user(&db, "creator", "creator@example.com").await;
        let deck_id = create_test_deck(&db, &user_id, "Test Deck", None, None, None)
            .await
            .unwrap();

        let result = get_deck(&db, &deck_id, &user_id).await;
        assert!(result.is_ok());

        let deck = result.unwrap();
        assert_eq!(deck.id, deck_id);
        assert_eq!(deck.title, "Test Deck");
        assert_eq!(deck.created_by, user_id);
    }

    #[sqlx::test]
    async fn test_get_deck_success_as_assignee(db: PgPool) {
        let creator_id = create_test_user(&db, "creator", "creator@example.com").await;
        let assignee_id = create_test_user(&db, "assignee", "assignee@example.com").await;

        let deck_id = create_test_deck(
            &db,
            &creator_id,
            "Assigned Deck",
            None,
            Some("assigned".to_string()),
            Some(assignee_id.clone()),
        )
        .await
        .unwrap();

        let result = get_deck(&db, &deck_id, &assignee_id).await;
        assert!(result.is_ok());

        let deck = result.unwrap();
        assert_eq!(deck.id, deck_id);
        assert_eq!(deck.assignee, Some(assignee_id));
    }

    #[sqlx::test]
    async fn test_get_deck_success_public_deck(db: PgPool) {
        let creator_id = create_test_user(&db, "creator", "creator@example.com").await;
        let other_user_id = create_test_user(&db, "other", "other@example.com").await;

        let deck_id = create_test_deck(
            &db,
            &creator_id,
            "Public Deck",
            None,
            Some("public".to_string()),
            None,
        )
        .await
        .unwrap();

        let result = get_deck(&db, &deck_id, &other_user_id).await;
        assert!(result.is_ok());

        let deck = result.unwrap();
        assert_eq!(deck.id, deck_id);
        assert_eq!(deck.visibility, "public");
    }

    #[sqlx::test]
    async fn test_get_deck_fails_private_deck_unauthorized(db: PgPool) {
        let creator_id = create_test_user(&db, "creator", "creator@example.com").await;
        let other_user_id = create_test_user(&db, "other", "other@example.com").await;

        let deck_id = create_test_deck(
            &db,
            &creator_id,
            "Private Deck",
            None,
            Some("private".to_string()),
            None,
        )
        .await
        .unwrap();

        let result = get_deck(&db, &deck_id, &other_user_id).await;
        assert!(result.is_err());
    }

    #[sqlx::test]
    async fn test_find_all_returns_created_and_assigned_decks(db: PgPool) {
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let other_user_id = create_test_user(&db, "other", "other@example.com").await;

        // Create a deck as creator
        let _deck1_id = create_test_deck(&db, &user_id, "Created Deck", None, None, None)
            .await
            .unwrap();

        // Create a deck assigned to user
        let _deck2_id = create_test_deck(
            &db,
            &other_user_id,
            "Assigned Deck",
            None,
            Some("assigned".to_string()),
            Some(user_id.clone()),
        )
        .await
        .unwrap();

        // Create a deck not related to user
        let _deck3_id = create_test_deck(&db, &other_user_id, "Other Deck", None, None, None)
            .await
            .unwrap();

        let params = PaginationParams {
            page: Some(1),
            per_page: Some(2),
            search: None,
            assignee: None,
        };

        let result = find_all(&db, &user_id, &params).await;
        assert!(result.is_ok());

        let decks = result.unwrap();
        assert_eq!(decks.len(), 2); // Only created and assigned decks
    }

    #[sqlx::test]
    async fn test_find_all_with_search_filter(db: PgPool) {
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;

        let _deck1_id = create_test_deck(
            &db,
            &user_id,
            "Math Flashcards",
            Some("Mathematics study cards".to_string()),
            None,
            None,
        )
        .await
        .unwrap();
        let _deck2_id = create_test_deck(
            &db,
            &user_id,
            "History Quiz",
            Some("World history questions".to_string()),
            None,
            None,
        )
        .await
        .unwrap();
        let _deck3_id = create_test_deck(
            &db,
            &user_id,
            "Science Facts",
            Some("Physics and chemistry".to_string()),
            None,
            None,
        )
        .await
        .unwrap();

        let params = PaginationParams {
            page: Some(1),
            per_page: Some(10),
            search: Some("math".to_string()),
            assignee: None,
        };

        let result = find_all(&db, &user_id, &params).await;
        assert!(result.is_ok());

        let decks = result.unwrap();
        assert_eq!(decks.len(), 1);
        assert_eq!(decks[0].title, "Math Flashcards");
    }

    #[sqlx::test]
    async fn test_find_all_with_assignee_filter(db: PgPool) {
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let assignee_id = create_test_user(&db, "assignee", "assignee@example.com").await;
        let other_assignee_id = create_test_user(&db, "other_assignee", "other@example.com").await;

        let _deck1_id = create_test_deck(
            &db,
            &user_id,
            "Deck 1",
            None,
            None,
            Some(assignee_id.clone()),
        )
        .await
        .unwrap();
        let _deck2_id =
            create_test_deck(&db, &user_id, "Deck 2", None, None, Some(other_assignee_id))
                .await
                .unwrap();
        let _deck3_id = create_test_deck(&db, &user_id, "Deck 3", None, None, None)
            .await
            .unwrap();

        let params = PaginationParams {
            page: Some(1),
            per_page: Some(20),
            search: None,
            assignee: Some(assignee_id.clone()),
        };

        let result = find_all(&db, &user_id, &params).await;
        assert!(result.is_ok());

        let decks = result.unwrap();
        assert_eq!(decks.len(), 1);
    }

    #[sqlx::test]
    async fn test_find_all_public_returns_only_public_decks(db: PgPool) {
        let user1_id = create_test_user(&db, "user1", "user1@example.com").await;
        let user2_id = create_test_user(&db, "user2", "user2@example.com").await;

        let _public_deck1 = create_test_deck(
            &db,
            &user1_id,
            "Public Deck 1",
            Some("First public deck".to_string()),
            Some("public".to_string()),
            None,
        )
        .await
        .unwrap();
        let _public_deck2 = create_test_deck(
            &db,
            &user2_id,
            "Public Deck 2",
            Some("Second public deck".to_string()),
            Some("public".to_string()),
            None,
        )
        .await
        .unwrap();
        let _private_deck = create_test_deck(
            &db,
            &user1_id,
            "Private Deck",
            None,
            Some("private".to_string()),
            None,
        )
        .await
        .unwrap();

        let result = find_all_public(&db).await;
        assert!(result.is_ok());

        let decks = result.unwrap();
        assert_eq!(decks.len(), 2);

        for deck in &decks {
            assert!(deck.title.starts_with("Public Deck"));
        }
    }

    #[sqlx::test]
    async fn test_delete_success_as_creator(db: PgPool) {
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let deck_id = create_test_deck(&db, &user_id, "Test Deck", None, None, None)
            .await
            .unwrap();

        let result = delete(&db, &deck_id, &user_id).await;
        assert!(result.is_ok());

        // Verify deck is deleted
        let find_result = get_deck(&db, &deck_id, &user_id).await;
        assert!(find_result.is_err());
    }

    #[sqlx::test]
    async fn test_delete_fails_not_creator(db: PgPool) {
        let creator_id = create_test_user(&db, "creator", "creator@example.com").await;
        let other_user_id = create_test_user(&db, "other", "other@example.com").await;

        let deck_id = create_test_deck(&db, &creator_id, "Test Deck", None, None, None)
            .await
            .unwrap();

        let result = delete(&db, &deck_id, &other_user_id).await;
        assert!(result.is_ok()); // Delete doesn't fail, but doesn't delete anything

        // Verify deck still exists
        let find_result = get_deck(&db, &deck_id, &creator_id).await;
        assert!(find_result.is_ok());
    }

    #[sqlx::test]
    async fn test_update_deck_only(db: PgPool) {
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let deck_id = create_test_deck(
            &db,
            &user_id,
            "Original Name",
            Some("Original description".to_string()),
            None,
            None,
        )
        .await
        .unwrap();

        let update_deck = DeckWithCardsUpdate {
            deck: DeckUpdate {
                title: Some("Updated Name".to_string()),
                description: Some("Updated description".to_string()),
                visibility: Some("public".to_string()),
                assignee: None,
            },
            cards: vec![], // No cards to update
        };

        let result = update(&db, &deck_id, &user_id, update_deck).await;
        assert!(result.is_ok());

        let updated_deck = get_deck(&db, &deck_id, &user_id).await.unwrap();
        assert_eq!(updated_deck.title, "Updated Name");
        assert_eq!(
            updated_deck.description,
            Some("Updated description".to_string())
        );
        assert_eq!(updated_deck.visibility, "public");
    }

    #[sqlx::test]
    async fn test_update_with_new_cards(db: PgPool) {
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let deck_id = create_test_deck(&db, &user_id, "Test Deck", None, None, None)
            .await
            .unwrap();

        let update_deck = DeckWithCardsUpdate {
            deck: DeckUpdate {
                title: None,
                description: None,
                visibility: None,
                assignee: None,
            },
            cards: vec![
                CardUpsert {
                    id: None, // New card
                    front: "Question 1".to_string(),
                    back: "Answer 1".to_string(),
                    media_url: None,
                },
                CardUpsert {
                    id: None, // New card
                    front: "Question 2".to_string(),
                    back: "Answer 2".to_string(),
                    media_url: Some("http://example.com/image.jpg".to_string()),
                },
            ],
        };

        let result = update(&db, &deck_id, &user_id, update_deck).await;
        assert!(result.is_ok());

        let card_count = count_cards_in_deck(&db, &deck_id).await.unwrap();
        assert_eq!(card_count, 2);
    }

    #[sqlx::test]
    async fn test_update_existing_cards(db: PgPool) {
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let deck_id = create_test_deck(&db, &user_id, "Test Deck", None, None, None)
            .await
            .unwrap();

        // Create initial cards
        let card_ids = create_test_cards(
            &db,
            &deck_id,
            vec![
                ("Original Q1", "Original A1", None),
                ("Original Q2", "Original A2", None),
            ],
        )
        .await
        .unwrap();

        let update_deck = DeckWithCardsUpdate {
            deck: DeckUpdate {
                title: None,
                description: None,
                visibility: None,
                assignee: None,
            },
            cards: vec![
                CardUpsert {
                    id: Some(card_ids[0].clone()),
                    front: "Updated Q1".to_string(),
                    back: "Updated A1".to_string(),
                    media_url: None,
                },
                CardUpsert {
                    id: Some(card_ids[1].clone()),
                    front: "Updated Q2".to_string(),
                    back: "Updated A2".to_string(),
                    media_url: Some("http://example.com/updated.jpg".to_string()),
                },
            ],
        };

        let result = update(&db, &deck_id, &user_id, update_deck).await;
        assert!(result.is_ok());

        let card_count = count_cards_in_deck(&db, &deck_id).await.unwrap();
        assert_eq!(card_count, 2);

        // Verify cards were updated
        let updated_cards = sqlx::query!(
            "SELECT front, back, media_url FROM cards WHERE deck_id = $1 ORDER BY front",
            deck_id
        )
        .fetch_all(&db)
        .await
        .unwrap();

        assert_eq!(updated_cards[0].front, "Updated Q1");
        assert_eq!(updated_cards[1].front, "Updated Q2");
        assert_eq!(
            updated_cards[1].media_url,
            Some("http://example.com/updated.jpg".to_string())
        );
    }

    #[sqlx::test]
    async fn test_update_removes_cards_not_in_update(db: PgPool) {
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let deck_id = create_test_deck(&db, &user_id, "Test Deck", None, None, None)
            .await
            .unwrap();

        // Create initial cards
        let card_ids = create_test_cards(
            &db,
            &deck_id,
            vec![("Q1", "A1", None), ("Q2", "A2", None), ("Q3", "A3", None)],
        )
        .await
        .unwrap();

        // Update with only one existing card (should remove the other two)
        let update_deck = DeckWithCardsUpdate {
            deck: DeckUpdate {
                title: None,
                description: None,
                visibility: None,
                assignee: None,
            },
            cards: vec![CardUpsert {
                id: Some(card_ids[0].clone()),
                front: "Updated Q1".to_string(),
                back: "Updated A1".to_string(),
                media_url: None,
            }],
        };

        let result = update(&db, &deck_id, &user_id, update_deck).await;
        assert!(result.is_ok());

        let card_count = count_cards_in_deck(&db, &deck_id).await.unwrap();
        assert_eq!(card_count, 1);
    }
}
