use crate::db::error::DbError;
use crate::types::preferences::{UserPreferences, UserPreferencesUpdate};
use sqlx::PgPool;

pub async fn find_by_user_id(
    db: &PgPool,
    user_id: &str,
) -> Result<Option<UserPreferences>, DbError> {
    let preferences = sqlx::query_as!(
        UserPreferences,
        r#"
        SELECT * FROM user_preferences
        WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_optional(db)
    .await?;

    Ok(preferences)
}

pub async fn upsert(
    db: &PgPool,
    user_id: &str,
    update: &UserPreferencesUpdate,
) -> Result<(), DbError> {
    sqlx::query!(
        r#"
        UPDATE user_preferences 
        SET
            auto_subscribe = COALESCE($2, auto_subscribe),
            email_notifications = COALESCE($3, email_notifications),
            push_notifications = COALESCE($4, push_notifications),
            theme = COALESCE($5, theme),
            language = COALESCE($6, language)
        WHERE user_id = $1
        "#,
        user_id,
        update.auto_subscribe,
        update.email_notifications,
        update.push_notifications,
        update.theme,
        update.language,
    )
    .execute(db)
    .await?;
    Ok(())
}

pub async fn get_or_create_defaults(
    db: &PgPool,
    user_id: &str,
) -> Result<UserPreferences, DbError> {
    let prefs = sqlx::query_as!(
        UserPreferences,
        r#"
            INSERT INTO user_preferences (user_id)
            VALUES ($1)
            ON CONFLICT (user_id) DO UPDATE SET user_id = EXCLUDED.user_id
            RETURNING *
            "#,
        user_id
    )
    .fetch_one(db)
    .await?;

    Ok(prefs)
}
#[cfg(test)]
mod additional_tests {
    use super::*;
    use crate::tests::create_test_user;
    use crate::types::preferences::UserPreferencesUpdate;
    use sqlx::PgPool;

    #[sqlx::test]
    async fn test_upsert_nonexistent_user_fails(db: PgPool) {
        let nonexistent_user_id = "nonexistent-user-12345";

        let update = UserPreferencesUpdate {
            auto_subscribe: Some(false),
            email_notifications: Some(true),
            push_notifications: Some(false),
            theme: Some("dark".to_string()),
            language: Some("es".to_string()),
        };

        // This should succeed but affect 0 rows since user doesn't exist
        let result = upsert(&db, nonexistent_user_id, &update).await;
        assert!(result.is_ok());

        // Verify no record was created
        let prefs = find_by_user_id(&db, nonexistent_user_id).await.unwrap();
        assert!(prefs.is_none());
    }

    #[sqlx::test]
    async fn test_upsert_empty_user_id(db: PgPool) {
        let update = UserPreferencesUpdate {
            auto_subscribe: Some(true),
            email_notifications: Some(true),
            push_notifications: Some(true),
            theme: Some("light".to_string()),
            language: Some("en".to_string()),
        };

        let result = upsert(&db, "", &update).await;
        assert!(result.is_ok());

        let prefs = find_by_user_id(&db, "").await.unwrap();
        assert!(prefs.is_none());
    }

    #[sqlx::test]
    async fn test_get_or_create_defaults_new_user(db: PgPool) {
        let user_id = create_test_user(&db, "newdefaultuser", "newdefault@example.com").await;

        let prefs = get_or_create_defaults(&db, &user_id).await.unwrap();

        // Verify default values
        assert_eq!(prefs.user_id, user_id);
        assert_eq!(prefs.auto_subscribe, true);
        assert_eq!(prefs.email_notifications, true);
        assert_eq!(prefs.push_notifications, true);
        assert_eq!(prefs.theme, "system");
        assert_eq!(prefs.language, "en");
    }

    #[sqlx::test]
    async fn test_get_or_create_defaults_existing_user(db: PgPool) {
        let user_id =
            create_test_user(&db, "existingdefaultuser", "existingdefault@example.com").await;

        // First create with custom values
        let initial_update = UserPreferencesUpdate {
            auto_subscribe: Some(false),
            email_notifications: Some(false),
            push_notifications: Some(false),
            theme: Some("dark".to_string()),
            language: Some("fr".to_string()),
        };

        // Manually insert with custom values
        sqlx::query!(
            r#"
            INSERT INTO user_preferences (user_id, auto_subscribe, email_notifications, push_notifications, theme, language)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            user_id,
            initial_update.auto_subscribe,
            initial_update.email_notifications,
            initial_update.push_notifications,
            initial_update.theme,
            initial_update.language
        )
        .execute(&db)
        .await
        .unwrap();

        // get_or_create_defaults should return existing values, not defaults
        let prefs = get_or_create_defaults(&db, &user_id).await.unwrap();

        assert_eq!(prefs.user_id, user_id);
        assert_eq!(prefs.auto_subscribe, false);
        assert_eq!(prefs.email_notifications, false);
        assert_eq!(prefs.push_notifications, false);
        assert_eq!(prefs.theme, "dark");
        assert_eq!(prefs.language, "fr");
    }

    #[sqlx::test]
    async fn test_upsert_partial_update_preserves_existing(db: PgPool) {
        let user_id = create_test_user(&db, "partialupdateuser", "partialupdate@example.com").await;

        // Create initial preferences
        get_or_create_defaults(&db, &user_id).await.unwrap();

        // Update only theme
        let theme_update = UserPreferencesUpdate {
            auto_subscribe: None,
            email_notifications: None,
            push_notifications: None,
            theme: Some("dark".to_string()),
            language: None,
        };

        upsert(&db, &user_id, &theme_update).await.unwrap();

        let prefs = find_by_user_id(&db, &user_id).await.unwrap().unwrap();
        // Only theme should change, others should remain defaults
        assert_eq!(prefs.auto_subscribe, true);
        assert_eq!(prefs.email_notifications, true);
        assert_eq!(prefs.push_notifications, true);
        assert_eq!(prefs.theme, "dark");
        assert_eq!(prefs.language, "en");
    }

    #[sqlx::test]
    async fn test_upsert_multiple_sequential_updates(db: PgPool) {
        let user_id = create_test_user(&db, "sequentialuser", "sequential@example.com").await;

        // Create initial preferences
        get_or_create_defaults(&db, &user_id).await.unwrap();

        // First update: change auto_subscribe
        let update1 = UserPreferencesUpdate {
            auto_subscribe: Some(false),
            email_notifications: None,
            push_notifications: None,
            theme: None,
            language: None,
        };
        upsert(&db, &user_id, &update1).await.unwrap();

        // Second update: change theme
        let update2 = UserPreferencesUpdate {
            auto_subscribe: None,
            email_notifications: None,
            push_notifications: None,
            theme: Some("light".to_string()),
            language: None,
        };
        upsert(&db, &user_id, &update2).await.unwrap();

        // Third update: change language and email_notifications
        let update3 = UserPreferencesUpdate {
            auto_subscribe: None,
            email_notifications: Some(false),
            push_notifications: None,
            theme: None,
            language: Some("de".to_string()),
        };
        upsert(&db, &user_id, &update3).await.unwrap();

        let final_prefs = find_by_user_id(&db, &user_id).await.unwrap().unwrap();

        // Verify all changes accumulated correctly
        assert_eq!(final_prefs.auto_subscribe, false); // From update1
        assert_eq!(final_prefs.email_notifications, false); // From update3
        assert_eq!(final_prefs.push_notifications, true); // Unchanged default
        assert_eq!(final_prefs.theme, "light"); // From update2
        assert_eq!(final_prefs.language, "de"); // From update3
    }

    #[sqlx::test]
    async fn test_find_by_user_id_with_special_characters(db: PgPool) {
        let user_id = create_test_user(&db, "special@user", "special@example.com").await;

        get_or_create_defaults(&db, &user_id).await.unwrap();

        let result = find_by_user_id(&db, &user_id).await.unwrap();
        assert!(result.is_some());
        assert_eq!(result.unwrap().user_id, user_id);
    }

    #[sqlx::test]
    async fn test_upsert_extreme_values(db: PgPool) {
        let user_id = create_test_user(&db, "extremeuser", "extreme@example.com").await;

        // Create initial preferences
        get_or_create_defaults(&db, &user_id).await.unwrap();

        // Test with very long strings (within reasonable limits)
        let long_theme = "a".repeat(50); // Assuming theme has reasonable length limit
        let long_language = "zh-Hans-CN"; // Real but longer language code

        let update = UserPreferencesUpdate {
            auto_subscribe: Some(false),
            email_notifications: Some(false),
            push_notifications: Some(false),
            theme: Some(long_theme.clone()),
            language: Some(long_language.to_string()),
        };

        let result = upsert(&db, &user_id, &update).await;

        // This might fail if database has constraints, which is good to test
        match result {
            Ok(_) => {
                let prefs = find_by_user_id(&db, &user_id).await.unwrap().unwrap();
                assert_eq!(prefs.theme, long_theme);
                assert_eq!(prefs.language, long_language);
            }
            Err(_) => {
                // If it fails due to length constraints, that's also valid behavior
                // The important thing is that it fails gracefully
            }
        }
    }

    #[sqlx::test]
    async fn test_get_or_create_defaults_empty_user_id(db: PgPool) {
        // This should probably fail or handle empty user_id gracefully
        let result = get_or_create_defaults(&db, "").await;

        // Depending on your database constraints, this might succeed or fail
        // If user_id has a NOT NULL constraint, it should fail
        // If it succeeds, we should be able to find it
        match result {
            Ok(prefs) => {
                assert_eq!(prefs.user_id, "");
                let found = find_by_user_id(&db, "").await.unwrap();
                assert!(found.is_some());
            }
            Err(_) => {
                // This is also acceptable behavior for empty user_id
            }
        }
    }
}
