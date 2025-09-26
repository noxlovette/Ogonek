use crate::DbError;
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
#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::create_test_user;
    use sqlx::PgPool;

    async fn create_test_deck(db: &PgPool, name: &str, user_id: &str) -> String {
        let deck_id = nanoid::nanoid!();
        sqlx::query!(
            r#"
            INSERT INTO decks (id, title, description, created_by)
            VALUES ($1, $2, $3, $4)
            "#,
            deck_id,
            name,
            "Test deck description",
            user_id
        )
        .execute(db)
        .await
        .unwrap();
        deck_id
    }

    async fn create_test_card(db: &PgPool, deck_id: &str, front: &str, back: &str) -> String {
        let card_id = nanoid::nanoid!();
        sqlx::query!(
            r#"
            INSERT INTO cards (id, front, back, deck_id)
            VALUES ($1, $2, $3, $4)
            "#,
            card_id,
            front,
            back,
            deck_id
        )
        .execute(db)
        .await
        .unwrap();
        card_id
    }

    async fn get_card_progress_count(db: &PgPool, user_id: &str, deck_id: &str) -> i64 {
        sqlx::query_scalar!(
            r#"
            SELECT COUNT(*) FROM card_progress cp
            JOIN cards c ON cp.card_id = c.id
            WHERE cp.user_id = $1 AND c.deck_id = $2
            "#,
            user_id,
            deck_id
        )
        .fetch_one(db)
        .await
        .unwrap()
        .unwrap_or(0)
    }

    #[sqlx::test]
    async fn test_subscribe_creates_subscription_and_card_progress(db: PgPool) {
        // Setup
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let deck_id = create_test_deck(&db, "Test Deck", &user_id).await;
        let _card1 = create_test_card(&db, &deck_id, "Front 1", "Back 1").await;
        let _card2 = create_test_card(&db, &deck_id, "Front 2", "Back 2").await;

        // Test
        let result = subscribe(&db, &deck_id, &user_id).await;

        // Verify
        assert!(result.is_ok());

        // Check subscription was created
        let is_subscribed = check_subscription(&db, &deck_id, &user_id).await.unwrap();
        assert!(is_subscribed);

        // Check card progress was created for all cards
        let progress_count = get_card_progress_count(&db, &user_id, &deck_id).await;
        assert_eq!(progress_count, 2);

        // Verify card progress details
        let progress = sqlx::query!(
            r#"
            SELECT review_count, ease_factor, interval FROM card_progress cp
            JOIN cards c ON cp.card_id = c.id
            WHERE cp.user_id = $1 AND c.deck_id = $2
            LIMIT 1
            "#,
            user_id,
            deck_id
        )
        .fetch_one(&db)
        .await
        .unwrap();

        assert_eq!(progress.review_count, 0);
        assert_eq!(progress.ease_factor, 2.5);
        assert_eq!(progress.interval, 1);
    }

    #[sqlx::test]
    async fn test_subscribe_handles_duplicate_subscription(db: PgPool) {
        // Setup
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let deck_id = create_test_deck(&db, "Test Deck", &user_id).await;
        let _card1 = create_test_card(&db, &deck_id, "Front 1", "Back 1").await;

        // Subscribe once
        subscribe(&db, &deck_id, &user_id).await.unwrap();

        // Test subscribing again
        let result = subscribe(&db, &deck_id, &user_id).await;

        // Verify - should not error
        assert!(result.is_ok());

        // Should still be subscribed
        let is_subscribed = check_subscription(&db, &deck_id, &user_id).await.unwrap();
        assert!(is_subscribed);

        // Should not create duplicate card progress
        let progress_count = get_card_progress_count(&db, &user_id, &deck_id).await;
        assert_eq!(progress_count, 1);
    }

    #[sqlx::test]
    async fn test_subscribe_with_existing_partial_progress(db: PgPool) {
        // Setup
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let deck_id = create_test_deck(&db, "Test Deck", &user_id).await;
        let card1 = create_test_card(&db, &deck_id, "Front 1", "Back 1").await;
        let _card2 = create_test_card(&db, &deck_id, "Front 2", "Back 2").await;

        // Create progress for only one card manually
        sqlx::query!(
            r#"
            INSERT INTO card_progress (id, user_id, card_id, review_count, due_date, ease_factor, interval)
            VALUES ($1, $2, $3, 5, CURRENT_TIMESTAMP, 3.0, 7)
            "#,
            nanoid::nanoid!(),
            user_id,
            card1
        )
        .execute(&db)
        .await
        .unwrap();

        // Test
        let result = subscribe(&db, &deck_id, &user_id).await;

        // Verify
        assert!(result.is_ok());

        // Should create progress for the missing card only
        let progress_count = get_card_progress_count(&db, &user_id, &deck_id).await;
        assert_eq!(progress_count, 2);

        // Verify existing progress wasn't modified
        let existing_progress = sqlx::query!(
            r#"
            SELECT review_count, ease_factor, interval FROM card_progress
            WHERE user_id = $1 AND card_id = $2
            "#,
            user_id,
            card1
        )
        .fetch_one(&db)
        .await
        .unwrap();

        assert_eq!(existing_progress.review_count, 5);
        assert_eq!(existing_progress.ease_factor, 3.0);
        assert_eq!(existing_progress.interval, 7);
    }

    #[sqlx::test]
    async fn test_subscribe_empty_deck(db: PgPool) {
        // Setup
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let deck_id = create_test_deck(&db, "Empty Deck", &user_id).await;

        // Test
        let result = subscribe(&db, &deck_id, &user_id).await;

        // Verify
        assert!(result.is_ok());

        // Should be subscribed even with no cards
        let is_subscribed = check_subscription(&db, &deck_id, &user_id).await.unwrap();
        assert!(is_subscribed);

        // No card progress should be created
        let progress_count = get_card_progress_count(&db, &user_id, &deck_id).await;
        assert_eq!(progress_count, 0);
    }

    #[sqlx::test]
    async fn test_unsubscribe_removes_subscription_and_progress(db: PgPool) {
        // Setup
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let deck_id = create_test_deck(&db, "Test Deck", &user_id).await;
        let _card1 = create_test_card(&db, &deck_id, "Front 1", "Back 1").await;
        let _card2 = create_test_card(&db, &deck_id, "Front 2", "Back 2").await;

        // Subscribe first
        subscribe(&db, &deck_id, &user_id).await.unwrap();

        // Verify setup
        assert!(check_subscription(&db, &deck_id, &user_id).await.unwrap());
        assert_eq!(get_card_progress_count(&db, &user_id, &deck_id).await, 2);

        // Test
        let result = unsubscribe(&db, &deck_id, &user_id).await;

        // Verify
        assert!(result.is_ok());

        // Should no longer be subscribed
        let is_subscribed = check_subscription(&db, &deck_id, &user_id).await.unwrap();
        assert!(!is_subscribed);

        // Card progress should be removed
        let progress_count = get_card_progress_count(&db, &user_id, &deck_id).await;
        assert_eq!(progress_count, 0);
    }

    #[sqlx::test]
    async fn test_unsubscribe_nonexistent_subscription(db: PgPool) {
        // Setup
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let deck_id = create_test_deck(&db, "Test Deck", &user_id).await;

        // Test unsubscribing without being subscribed
        let result = unsubscribe(&db, &deck_id, &user_id).await;

        // Verify - should not error
        assert!(result.is_ok());

        // Should still not be subscribed
        let is_subscribed = check_subscription(&db, &deck_id, &user_id).await.unwrap();
        assert!(!is_subscribed);
    }

    #[sqlx::test]
    async fn test_unsubscribe_preserves_other_user_progress(db: PgPool) {
        // Setup
        let user1_id = create_test_user(&db, "user1", "user1@example.com").await;
        let user2_id = create_test_user(&db, "user2", "user2@example.com").await;
        let deck_id = create_test_deck(&db, "Test Deck", &user1_id).await;
        let _card1 = create_test_card(&db, &deck_id, "Front 1", "Back 1").await;

        // Both users subscribe
        subscribe(&db, &deck_id, &user1_id).await.unwrap();
        subscribe(&db, &deck_id, &user2_id).await.unwrap();

        // Verify both have progress
        assert_eq!(get_card_progress_count(&db, &user1_id, &deck_id).await, 1);
        assert_eq!(get_card_progress_count(&db, &user2_id, &deck_id).await, 1);

        // User1 unsubscribes
        unsubscribe(&db, &deck_id, &user1_id).await.unwrap();

        // Verify user1's progress is gone but user2's remains
        assert_eq!(get_card_progress_count(&db, &user1_id, &deck_id).await, 0);
        assert_eq!(get_card_progress_count(&db, &user2_id, &deck_id).await, 1);
        assert!(!check_subscription(&db, &deck_id, &user1_id).await.unwrap());
        assert!(check_subscription(&db, &deck_id, &user2_id).await.unwrap());
    }

    #[sqlx::test]
    async fn test_check_subscription_returns_correct_status(db: PgPool) {
        // Setup
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let deck_id = create_test_deck(&db, "Test Deck", &user_id).await;

        // Test - not subscribed initially
        let is_subscribed = check_subscription(&db, &deck_id, &user_id).await.unwrap();
        assert!(!is_subscribed);

        // Subscribe
        subscribe(&db, &deck_id, &user_id).await.unwrap();

        // Test - should be subscribed now
        let is_subscribed = check_subscription(&db, &deck_id, &user_id).await.unwrap();
        assert!(is_subscribed);

        // Unsubscribe
        unsubscribe(&db, &deck_id, &user_id).await.unwrap();

        // Test - should not be subscribed again
        let is_subscribed = check_subscription(&db, &deck_id, &user_id).await.unwrap();
        assert!(!is_subscribed);
    }

    #[sqlx::test]
    async fn test_check_subscription_with_invalid_ids(db: PgPool) {
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let fake_deck_id = nanoid::nanoid!();

        // Test with non-existent deck
        let result = check_subscription(&db, &fake_deck_id, &user_id).await;
        assert!(result.is_ok());
        assert!(!result.unwrap());

        // Test with non-existent user
        let deck_id = create_test_deck(&db, "Test Deck", &user_id).await;
        let fake_user_id = nanoid::nanoid!();
        let result = check_subscription(&db, &deck_id, &fake_user_id).await;
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[sqlx::test]
    async fn test_subscribe_adds_progress_for_cards_added_after_subscription(db: PgPool) {
        // Setup
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let deck_id = create_test_deck(&db, "Test Deck", &user_id).await;
        let _card1 = create_test_card(&db, &deck_id, "Front 1", "Back 1").await;

        // Subscribe with one card
        subscribe(&db, &deck_id, &user_id).await.unwrap();
        assert_eq!(get_card_progress_count(&db, &user_id, &deck_id).await, 1);

        // Add another card
        let _card2 = create_test_card(&db, &deck_id, "Front 2", "Back 2").await;

        // Subscribe again (should add progress for new card)
        let result = subscribe(&db, &deck_id, &user_id).await;
        assert!(result.is_ok());

        // Should now have progress for both cards
        assert_eq!(get_card_progress_count(&db, &user_id, &deck_id).await, 2);
    }
}
