use crate::db::error::DbError;
use crate::models::{
    CreationId, Deck, DeckCreate, DeckFilterParams, DeckSmall, DeckWithCardsUpdate,
};
use sqlx::PgPool;

/// Find all decks WITHOUT cards, with subscription status
pub async fn find_all(
    db: &PgPool,
    user_id: &str,
    params: &DeckFilterParams,
) -> Result<Vec<Deck>, DbError> {
    let mut query_builder = sqlx::QueryBuilder::new(
        "SELECT d.id, d.name, d.description, d.visibility, d.assignee, d.created_by, d.created_at,
                EXISTS (
                    SELECT 1 FROM deck_subscriptions s 
                    WHERE s.deck_id = d.id AND s.user_id = ",
    );
    query_builder.push_bind(user_id);
    query_builder.push(
        ") as is_subscribed
        FROM decks d
        WHERE (d.created_by = ",
    );
    query_builder.push_bind(user_id);
    query_builder.push(" OR d.assignee = ");
    query_builder.push_bind(user_id);
    query_builder.push(")");

    if let Some(search) = &params.search {
        query_builder.push(" AND (name ILIKE ");
        query_builder.push_bind(format!("%{search}%"));
        query_builder.push(" OR description ILIKE ");
        query_builder.push_bind(format!("%{search}%"));
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

/// Find a deck by id WITH cards and subscription status included
pub async fn find_by_id(db: &PgPool, deck_id: &str, user_id: &str) -> Result<Deck, DbError> {
    let deck = sqlx::query_as!(
        Deck,
        r#"
        SELECT 
        d.id, 
        d.name, 
        d.description, 
        d.visibility, 
        d.assignee, 
        d.created_by, 
        d.created_at,
        EXISTS (
            SELECT 1 FROM deck_subscriptions
            WHERE deck_id = d.id AND user_id = $2
        ) AS "is_subscribed!"
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

/// Returns three decks
pub async fn find_recent(db: &PgPool, user_id: &str) -> Result<Vec<Deck>, DbError> {
    let decks = sqlx::query_as!(
        Deck,
        r#"
        SELECT 
        d.id, 
        d.name, 
        d.description, 
        d.visibility, 
        d.assignee, 
        d.created_by, 
        d.created_at,
        EXISTS (
            SELECT 1 FROM deck_subscriptions s
            WHERE s.deck_id = d.id AND user_id = $1
        ) AS "is_subscribed!"
    FROM decks d
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

/// Creates a new deck
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

/// Deletes a deck
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

/// Updates a deck
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

/// Count the overall number of deck
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
#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{CardUpdate, DeckUpdate};
    use crate::tests::create_test_user;
    use sqlx::PgPool;

    // Helper function to create a test deck
    async fn create_test_deck(
        db: &PgPool,
        user_id: &str,
        name: &str,
        description: Option<String>,
        visibility: Option<String>,
        assignee: Option<String>,
    ) -> Result<String, DbError> {
        let deck_create = DeckCreate {
            name: name.to_string(),
            description,
            visibility,
            assignee,
        };
        let result = create(db, user_id, deck_create).await?;
        Ok(result.id)
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

    // Helper function to count cards in a deck
    async fn count_cards_in_deck(db: &PgPool, deck_id: &str) -> Result<i64, DbError> {
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
            name: "Test Deck".to_string(),
            description: Some("A test deck".to_string()),
            visibility: Some("private".to_string()),
            assignee: None,
        };

        let result = create(&db, &user_id, deck_create).await;
        assert!(result.is_ok());

        let deck_id = result.unwrap().id;
        assert!(!deck_id.is_empty());
    }

    #[sqlx::test]
    async fn test_create_deck_with_assignee_defaults_to_assigned_visibility(db: PgPool) {
        let user_id = create_test_user(&db, "creator", "creator@example.com").await;
        let assignee_id = create_test_user(&db, "assignee", "assignee@example.com").await;

        let deck_create = DeckCreate {
            name: "Assigned Deck".to_string(),
            description: None,
            visibility: None, // Should default to "assigned"
            assignee: Some(assignee_id.clone()),
        };

        let result = create(&db, &user_id, deck_create).await;
        assert!(result.is_ok());

        let deck_id = result.unwrap().id;
        let deck = find_by_id(&db, &deck_id, &user_id).await.unwrap();
        assert_eq!(deck.visibility, "assigned");
        assert_eq!(deck.assignee, Some(assignee_id));
    }

    #[sqlx::test]
    async fn test_create_deck_without_assignee_defaults_to_private_visibility(db: PgPool) {
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;

        let deck_create = DeckCreate {
            name: "Private Deck".to_string(),
            description: None,
            visibility: None, // Should default to "private"
            assignee: None,
        };

        let result = create(&db, &user_id, deck_create).await;
        assert!(result.is_ok());

        let deck_id = result.unwrap().id;
        let deck = find_by_id(&db, &deck_id, &user_id).await.unwrap();
        assert_eq!(deck.visibility, "private");
        assert_eq!(deck.assignee, None);
    }

    #[sqlx::test]
    async fn test_find_by_id_success_as_creator(db: PgPool) {
        let user_id = create_test_user(&db, "creator", "creator@example.com").await;
        let deck_id = create_test_deck(&db, &user_id, "Test Deck", None, None, None)
            .await
            .unwrap();

        let result = find_by_id(&db, &deck_id, &user_id).await;
        assert!(result.is_ok());

        let deck = result.unwrap();
        assert_eq!(deck.id, deck_id);
        assert_eq!(deck.name, "Test Deck");
        assert_eq!(deck.created_by, user_id);
    }

    #[sqlx::test]
    async fn test_find_by_id_success_as_assignee(db: PgPool) {
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

        let result = find_by_id(&db, &deck_id, &assignee_id).await;
        assert!(result.is_ok());

        let deck = result.unwrap();
        assert_eq!(deck.id, deck_id);
        assert_eq!(deck.assignee, Some(assignee_id));
    }

    #[sqlx::test]
    async fn test_find_by_id_success_public_deck(db: PgPool) {
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

        let result = find_by_id(&db, &deck_id, &other_user_id).await;
        assert!(result.is_ok());

        let deck = result.unwrap();
        assert_eq!(deck.id, deck_id);
        assert_eq!(deck.visibility, "public");
    }

    #[sqlx::test]
    async fn test_find_by_id_fails_private_deck_unauthorized(db: PgPool) {
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

        let result = find_by_id(&db, &deck_id, &other_user_id).await;
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

        let params = DeckFilterParams {
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

        let params = DeckFilterParams {
            search: Some("math".to_string()),
            assignee: None,
        };

        let result = find_all(&db, &user_id, &params).await;
        assert!(result.is_ok());

        let decks = result.unwrap();
        assert_eq!(decks.len(), 1);
        assert_eq!(decks[0].name, "Math Flashcards");
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

        let params = DeckFilterParams {
            search: None,
            assignee: Some(assignee_id.clone()),
        };

        let result = find_all(&db, &user_id, &params).await;
        assert!(result.is_ok());

        let decks = result.unwrap();
        assert_eq!(decks.len(), 1);
        assert_eq!(decks[0].assignee, Some(assignee_id));
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
            assert!(deck.name.starts_with("Public Deck"));
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
        let find_result = find_by_id(&db, &deck_id, &user_id).await;
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
        let find_result = find_by_id(&db, &deck_id, &creator_id).await;
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
                name: Some("Updated Name".to_string()),
                description: Some("Updated description".to_string()),
                visibility: Some("public".to_string()),
                assignee: None,
            },
            cards: vec![], // No cards to update
        };

        let result = update(&db, &deck_id, &user_id, update_deck).await;
        assert!(result.is_ok());

        let updated_deck = find_by_id(&db, &deck_id, &user_id).await.unwrap();
        assert_eq!(updated_deck.name, "Updated Name");
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
                name: None,
                description: None,
                visibility: None,
                assignee: None,
            },
            cards: vec![
                CardUpdate {
                    id: None, // New card
                    front: Some("Question 1".to_string()),
                    back: Some("Answer 1".to_string()),
                    media_url: None,
                },
                CardUpdate {
                    id: None, // New card
                    front: Some("Question 2".to_string()),
                    back: Some("Answer 2".to_string()),
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
                name: None,
                description: None,
                visibility: None,
                assignee: None,
            },
            cards: vec![
                CardUpdate {
                    id: Some(card_ids[0].clone()),
                    front: Some("Updated Q1".to_string()),
                    back: Some("Updated A1".to_string()),
                    media_url: None,
                },
                CardUpdate {
                    id: Some(card_ids[1].clone()),
                    front: Some("Updated Q2".to_string()),
                    back: Some("Updated A2".to_string()),
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
                name: None,
                description: None,
                visibility: None,
                assignee: None,
            },
            cards: vec![CardUpdate {
                id: Some(card_ids[0].clone()),
                front: Some("Updated Q1".to_string()),
                back: Some("Updated A1".to_string()),
                media_url: None,
            }],
        };

        let result = update(&db, &deck_id, &user_id, update_deck).await;
        assert!(result.is_ok());

        let card_count = count_cards_in_deck(&db, &deck_id).await.unwrap();
        assert_eq!(card_count, 1);
    }
}
