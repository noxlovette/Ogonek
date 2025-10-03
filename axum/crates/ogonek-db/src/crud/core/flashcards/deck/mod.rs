mod create;
mod read;
pub use create::*;
pub use read::*;
mod delete;
pub use delete::*;
mod update;
pub use update::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        DbError,
        core::flashcards::card::{self, batch_upsert},
        tests::create_test_user,
    };
    use ogonek_types::{CardUpsert, DeckCreate, DeckUpdate, DeckWithCardsUpdate};
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
        let deck = read_deck(&db, &deck_id, &user_id).await.unwrap();
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
        let deck = read_deck(&db, &deck_id, &user_id).await.unwrap();
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
        let new_deck = read_deck(&db, &new_deck_id, &user_id).await.unwrap();
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
        let new_deck = read_deck(&db, &new_deck_id, &user_id).await.unwrap();
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
        // This should fail because read_deck checks ownership
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
            let copied_deck = read_deck(&db, &result.unwrap(), &user_id).await.unwrap();
            assert_eq!(copied_deck.title, expected_copy_name);
        }
    }

    #[sqlx::test]
    async fn test_read_deck_success_as_creator(db: PgPool) {
        let user_id = create_test_user(&db, "creator", "creator@example.com").await;
        let deck_id = create_test_deck(&db, &user_id, "Test Deck", None, None, None)
            .await
            .unwrap();

        let result = read_deck(&db, &deck_id, &user_id).await;
        assert!(result.is_ok());

        let deck = result.unwrap();
        assert_eq!(deck.id, deck_id);
        assert_eq!(deck.title, "Test Deck");
        assert_eq!(deck.created_by, user_id);
    }

    #[sqlx::test]
    async fn test_read_deck_success_as_assignee(db: PgPool) {
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

        let result = read_deck(&db, &deck_id, &assignee_id).await;
        assert!(result.is_ok());

        let deck = result.unwrap();
        assert_eq!(deck.id, deck_id);
        assert_eq!(deck.assignee, Some(assignee_id));
    }

    #[sqlx::test]
    async fn test_read_deck_success_public_deck(db: PgPool) {
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

        let result = read_deck(&db, &deck_id, &other_user_id).await;
        assert!(result.is_ok());

        let deck = result.unwrap();
        assert_eq!(deck.id, deck_id);
        assert_eq!(deck.visibility, "public");
    }

    #[sqlx::test]
    async fn test_read_deck_fails_private_deck_unauthorized(db: PgPool) {
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

        let result = read_deck(&db, &deck_id, &other_user_id).await;
        assert!(result.is_err());
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

        let result = read_all_public(&db).await;
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
        let find_result = read_deck(&db, &deck_id, &user_id).await;
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
        let find_result = read_deck(&db, &deck_id, &creator_id).await;
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

        let updated_deck = read_deck(&db, &deck_id, &user_id).await.unwrap();
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
