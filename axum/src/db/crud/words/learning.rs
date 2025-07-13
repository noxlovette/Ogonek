use crate::db::error::DbError;
use crate::models::{CardProgress, CardProgressWithFields, SimpleStats, UpdateCardProgress};
use sqlx::PgPool;

pub async fn fetch_due(db: &PgPool, user_id: &str) -> Result<Vec<CardProgressWithFields>, DbError> {
    let due = sqlx::query_as!(
        CardProgressWithFields,
        r#"
        SELECT
            cp.*,
            c.front,
            c.back,
            c.media_url
        FROM card_progress cp
        JOIN cards c ON c.id = cp.card_id
        WHERE cp.user_id = $1
            AND (cp.due_date <= CURRENT_TIMESTAMP OR cp.review_count = 0)
        ORDER BY cp.due_date ASC
        "#,
        user_id,
    )
    .fetch_all(db)
    .await?;

    Ok(due)
}

pub async fn fetch_due_count(db: &PgPool, user_id: &str) -> Result<Option<i64>, DbError> {
    let due = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*)
        FROM card_progress cp
        WHERE cp.user_id = $1
            AND (cp.due_date <= CURRENT_TIMESTAMP OR cp.review_count = 0)
        "#,
        user_id,
    )
    .fetch_one(db)
    .await?;

    Ok(due)
}

pub async fn get_simple_stats(db: &PgPool, user_id: &str) -> Result<SimpleStats, DbError> {
    // Cards studied today
    let cards_today = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*)::int
        FROM card_progress 
        WHERE user_id = $1 
        AND last_reviewed::date = CURRENT_DATE
        "#,
        user_id
    )
    .fetch_one(db)
    .await?
    .unwrap_or(0);

    // Current streak - count consecutive days with reviews
    let streak = sqlx::query_scalar!(
        r#"
        WITH daily_reviews AS (
            SELECT DISTINCT last_reviewed::date as review_date
            FROM card_progress 
            WHERE user_id = $1 
            AND last_reviewed IS NOT NULL
            ORDER BY review_date DESC
        ),
        consecutive_days AS (
            SELECT 
                review_date,
                ROW_NUMBER() OVER (ORDER BY review_date DESC) as rn,
                review_date - (ROW_NUMBER() OVER (ORDER BY review_date DESC) * interval '1 day') as group_date
            FROM daily_reviews
            WHERE review_date >= CURRENT_DATE - interval '365 days'  -- reasonable limit
        )
        SELECT COUNT(*)::int
        FROM consecutive_days
        WHERE group_date = (
            SELECT group_date 
            FROM consecutive_days 
            WHERE review_date = CURRENT_DATE 
            OR review_date = CURRENT_DATE - interval '1 day'
            LIMIT 1
        )
        "#,
        user_id
    )
    .fetch_one(db)
    .await?
    .unwrap_or(0);

    Ok(SimpleStats {
        cards_studied_today: cards_today,
        current_streak: streak,
    })
}
pub async fn find_by_id(
    db: &PgPool,
    progress_id: &str,
    user_id: &str,
) -> Result<CardProgress, DbError> {
    let progress = sqlx::query_as!(
        CardProgress,
        r#"
        SELECT cp.* FROM card_progress cp
        WHERE cp.user_id = $1 AND cp.id = $2
        "#,
        user_id,
        progress_id
    )
    .fetch_one(db)
    .await?;

    Ok(progress)
}

pub async fn update(
    db: &PgPool,
    card_id: &str,
    user_id: &str,
    update: UpdateCardProgress,
) -> Result<(), DbError> {
    sqlx::query!(
        r#"
        UPDATE card_progress SET
            review_count = $1,
            ease_factor = $2,
            interval = $3,
            last_reviewed = $4,
            due_date = $5
        WHERE user_id = $6 AND id = $7
        "#,
        update.review_count,
        update.ease_factor,
        update.interval,
        update.last_reviewed,
        update.due_date,
        user_id,
        card_id
    )
    .execute(db)
    .await?;

    Ok(())
}

pub async fn reset(db: &PgPool, deck_id: &str, user_id: &str) -> Result<(), DbError> {
    let mut tx = db.begin().await?;

    sqlx::query!(
        r#"
        UPDATE card_progress cp
        SET
            review_count = 0,
            ease_factor = 2.5,
            interval = 1,
            last_reviewed = NULL,
            due_date = CURRENT_TIMESTAMP
        FROM cards c
        WHERE cp.card_id = c.id
        AND c.deck_id = $1
        AND cp.user_id = $2
        "#,
        deck_id,
        user_id
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::create_test_user;
    use sqlx::PgPool;
    use time::OffsetDateTime;

    // Helper function to create a test deck
    async fn create_test_deck(db: &PgPool, user_id: &str, name: &str) -> String {
        let deck_id = nanoid::nanoid!();
        sqlx::query!(
            "INSERT INTO decks (id, name, created_by) VALUES ($1, $2, $3)",
            deck_id,
            name,
            user_id
        )
        .execute(db)
        .await
        .unwrap();
        deck_id
    }

    // Helper function to create a test card
    async fn create_test_card(db: &PgPool, deck_id: &str, front: &str, back: &str) -> String {
        let card_id = nanoid::nanoid!();
        sqlx::query!(
            "INSERT INTO cards (id, front, back, deck_id) VALUES ($1, $2, $3, $4)",
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

    // Helper function to create card progress
    async fn create_card_progress(
        db: &PgPool,
        user_id: &str,
        card_id: &str,
        review_count: i32,
        due_date: Option<OffsetDateTime>,
    ) -> String {
        let progress_id = nanoid::nanoid!();
        let due_date = due_date.unwrap_or_else(|| OffsetDateTime::now_utc());

        sqlx::query!(
            r#"
            INSERT INTO card_progress (id, user_id, card_id, review_count, due_date, ease_factor, interval)
            VALUES ($1, $2, $3, $4, $5, 2.5, 1)
            "#,
            progress_id,
            user_id,
            card_id,
            review_count,
            due_date
        )
        .execute(db)
        .await
        .unwrap();
        progress_id
    }

    #[sqlx::test]
    async fn test_fetch_due_returns_due_cards(db: PgPool) {
        // Setup
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let deck_id = create_test_deck(&db, &user_id, "Test Deck").await;
        let card1_id = create_test_card(&db, &deck_id, "Front 1", "Back 1").await;
        let card2_id = create_test_card(&db, &deck_id, "Front 2", "Back 2").await;

        // Create one due card and one not due card
        let past_date = OffsetDateTime::now_utc() - time::Duration::hours(1);
        let future_date = OffsetDateTime::now_utc() + time::Duration::hours(1);

        create_card_progress(&db, &user_id, &card1_id, 1, Some(past_date)).await;
        create_card_progress(&db, &user_id, &card2_id, 1, Some(future_date)).await;

        // Test
        let result = fetch_due(&db, &user_id).await.unwrap();

        // Assert
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].card_id, card1_id);
        assert_eq!(result[0].front, "Front 1");
        assert_eq!(result[0].back, "Back 1");
    }

    #[sqlx::test]
    async fn test_fetch_due_includes_new_cards(db: PgPool) {
        // Setup
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let deck_id = create_test_deck(&db, &user_id, "Test Deck").await;
        let card_id = create_test_card(&db, &deck_id, "New Card", "New Back").await;

        // Create card progress with review_count = 0 (new card)
        create_card_progress(&db, &user_id, &card_id, 0, None).await;

        // Test
        let result = fetch_due(&db, &user_id).await.unwrap();

        // Assert
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].review_count, 0);
        assert_eq!(result[0].front, "New Card");
    }

    #[sqlx::test]
    async fn test_fetch_due_orders_by_due_date(db: PgPool) {
        // Setup
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let deck_id = create_test_deck(&db, &user_id, "Test Deck").await;
        let card1_id = create_test_card(&db, &deck_id, "Card 1", "Back 1").await;
        let card2_id = create_test_card(&db, &deck_id, "Card 2", "Back 2").await;

        // Create cards with different due dates (both in the past)
        let earlier_date = OffsetDateTime::now_utc() - time::Duration::hours(2);
        let later_date = OffsetDateTime::now_utc() - time::Duration::hours(1);

        create_card_progress(&db, &user_id, &card1_id, 1, Some(later_date)).await;
        create_card_progress(&db, &user_id, &card2_id, 1, Some(earlier_date)).await;

        // Test
        let result = fetch_due(&db, &user_id).await.unwrap();

        // Assert - should be ordered by due_date ASC (earlier first)
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].card_id, card2_id); // Earlier due date first
        assert_eq!(result[1].card_id, card1_id);
    }

    #[sqlx::test]
    async fn test_fetch_due_empty_for_no_due_cards(db: PgPool) {
        // Setup
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let deck_id = create_test_deck(&db, &user_id, "Test Deck").await;
        let card_id = create_test_card(&db, &deck_id, "Future Card", "Future Back").await;

        // Create card with future due date
        let future_date = OffsetDateTime::now_utc() + time::Duration::hours(1);
        create_card_progress(&db, &user_id, &card_id, 1, Some(future_date)).await;

        // Test
        let result = fetch_due(&db, &user_id).await.unwrap();

        // Assert
        assert_eq!(result.len(), 0);
    }

    #[sqlx::test]
    async fn test_find_by_id_returns_correct_progress(db: PgPool) {
        // Setup
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let deck_id = create_test_deck(&db, &user_id, "Test Deck").await;
        let card_id = create_test_card(&db, &deck_id, "Test Card", "Test Back").await;
        let progress_id = create_card_progress(&db, &user_id, &card_id, 5, None).await;

        // Test
        let result = find_by_id(&db, &progress_id, &user_id).await.unwrap();

        // Assert
        assert_eq!(result.id, progress_id);
        assert_eq!(result.user_id, user_id);
        assert_eq!(result.card_id, card_id);
        assert_eq!(result.review_count, 5);
    }

    #[sqlx::test]
    async fn test_find_by_id_fails_for_wrong_user(db: PgPool) {
        // Setup
        let user1_id = create_test_user(&db, "user1", "user1@example.com").await;
        let user2_id = create_test_user(&db, "user2", "user2@example.com").await;
        let deck_id = create_test_deck(&db, &user1_id, "Test Deck").await;
        let card_id = create_test_card(&db, &deck_id, "Test Card", "Test Back").await;
        let progress_id = create_card_progress(&db, &user1_id, &card_id, 5, None).await;

        // Test - try to access user1's progress as user2
        let result = find_by_id(&db, &progress_id, &user2_id).await;

        // Assert
        assert!(result.is_err());
    }

    #[sqlx::test]
    async fn test_update_modifies_card_progress(db: PgPool) {
        // Setup
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let deck_id = create_test_deck(&db, &user_id, "Test Deck").await;
        let card_id = create_test_card(&db, &deck_id, "Test Card", "Test Back").await;
        let progress_id = create_card_progress(&db, &user_id, &card_id, 0, None).await;

        let new_due_date = OffsetDateTime::now_utc() + time::Duration::days(3);
        let last_reviewed = OffsetDateTime::now_utc();

        let update_card = UpdateCardProgress {
            review_count: 3,
            ease_factor: 2.8,
            interval: 7,
            last_reviewed,
            due_date: new_due_date,
        };

        // Test
        let result = update(&db, &progress_id, &user_id, update_card).await;
        assert!(result.is_ok());

        // Verify the update
        let updated_progress = find_by_id(&db, &progress_id, &user_id).await.unwrap();
        assert_eq!(updated_progress.review_count, 3);
        assert_eq!(updated_progress.ease_factor, 2.8);
        assert_eq!(updated_progress.interval, 7);
        assert!(updated_progress.last_reviewed.is_some());
        assert_eq!(
            updated_progress
                .due_date
                .expect("due time should be set")
                .unix_timestamp(),
            new_due_date.unix_timestamp()
        );
    }

    #[sqlx::test]
    async fn test_reset_resets_all_deck_progress(db: PgPool) {
        // Setup
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let deck_id = create_test_deck(&db, &user_id, "Test Deck").await;
        let card1_id = create_test_card(&db, &deck_id, "Card 1", "Back 1").await;
        let card2_id = create_test_card(&db, &deck_id, "Card 2", "Back 2").await;

        // Create progress with advanced states
        let progress1_id = create_card_progress(&db, &user_id, &card1_id, 10, None).await;
        let progress2_id = create_card_progress(&db, &user_id, &card2_id, 5, None).await;

        // Modify the progress to have advanced values
        sqlx::query!(
            r#"
            UPDATE card_progress 
            SET review_count = 10, ease_factor = 3.0, interval = 30, 
                last_reviewed = CURRENT_TIMESTAMP, due_date = CURRENT_TIMESTAMP + INTERVAL '30 days'
            WHERE id = $1
            "#,
            progress1_id
        )
        .execute(&db)
        .await
        .unwrap();

        sqlx::query!(
            r#"
            UPDATE card_progress 
            SET review_count = 5, ease_factor = 2.8, interval = 15, 
                last_reviewed = CURRENT_TIMESTAMP, due_date = CURRENT_TIMESTAMP + INTERVAL '15 days'
            WHERE id = $1
            "#,
            progress2_id
        )
        .execute(&db)
        .await
        .unwrap();

        // Test
        let result = reset(&db, &deck_id, &user_id).await;
        assert!(result.is_ok());

        // Verify reset values
        let progress1 = find_by_id(&db, &progress1_id, &user_id).await.unwrap();
        let progress2 = find_by_id(&db, &progress2_id, &user_id).await.unwrap();

        // Both should be reset to initial values
        assert_eq!(progress1.review_count, 0);
        assert_eq!(progress1.ease_factor, 2.5);
        assert_eq!(progress1.interval, 1);
        assert!(progress1.last_reviewed.is_none());

        assert_eq!(progress2.review_count, 0);
        assert_eq!(progress2.ease_factor, 2.5);
        assert_eq!(progress2.interval, 1);
        assert!(progress2.last_reviewed.is_none());
    }

    #[sqlx::test]
    async fn test_reset_only_affects_specific_deck(db: PgPool) {
        // Setup
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let deck1_id = create_test_deck(&db, &user_id, "Deck 1").await;
        let deck2_id = create_test_deck(&db, &user_id, "Deck 2").await;

        let card1_id = create_test_card(&db, &deck1_id, "Card 1", "Back 1").await;
        let card2_id = create_test_card(&db, &deck2_id, "Card 2", "Back 2").await;

        let progress1_id = create_card_progress(&db, &user_id, &card1_id, 5, None).await;
        let progress2_id = create_card_progress(&db, &user_id, &card2_id, 8, None).await;

        // Test - reset only deck1
        let result = reset(&db, &deck1_id, &user_id).await;
        assert!(result.is_ok());

        // Verify only deck1 progress was reset
        let progress1 = find_by_id(&db, &progress1_id, &user_id).await.unwrap();
        let progress2 = find_by_id(&db, &progress2_id, &user_id).await.unwrap();

        assert_eq!(progress1.review_count, 0); // Reset
        assert_eq!(progress2.review_count, 8); // Unchanged
    }

    #[sqlx::test]
    async fn test_reset_only_affects_specific_user(db: PgPool) {
        // Setup
        let user1_id = create_test_user(&db, "user1", "user1@example.com").await;
        let user2_id = create_test_user(&db, "user2", "user2@example.com").await;
        let deck_id = create_test_deck(&db, &user1_id, "Shared Deck").await;
        let card_id = create_test_card(&db, &deck_id, "Shared Card", "Shared Back").await;

        // Both users have progress on the same card
        let progress1_id = create_card_progress(&db, &user1_id, &card_id, 5, None).await;
        let progress2_id = create_card_progress(&db, &user2_id, &card_id, 8, None).await;

        // Test - reset only user1's progress
        let result = reset(&db, &deck_id, &user1_id).await;
        assert!(result.is_ok());

        // Verify only user1's progress was reset
        let progress1 = find_by_id(&db, &progress1_id, &user1_id).await.unwrap();
        let progress2 = find_by_id(&db, &progress2_id, &user2_id).await.unwrap();

        assert_eq!(progress1.review_count, 0); // Reset
        assert_eq!(progress2.review_count, 8); // Unchanged
    }

    #[sqlx::test]
    async fn test_fetch_due_includes_media_url(db: PgPool) {
        // Setup
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let deck_id = create_test_deck(&db, &user_id, "Test Deck").await;

        // Create card with media URL
        let card_id = nanoid::nanoid!();
        let media_url = "https://example.com/image.jpg";
        sqlx::query!(
            "INSERT INTO cards (id, front, back, deck_id, media_url) VALUES ($1, $2, $3, $4, $5)",
            card_id,
            "Front with media",
            "Back with media",
            deck_id,
            media_url
        )
        .execute(&db)
        .await
        .unwrap();

        create_card_progress(&db, &user_id, &card_id, 0, None).await;

        // Test
        let result = fetch_due(&db, &user_id).await.unwrap();

        // Assert
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].media_url, Some(media_url.to_string()));
    }
}
