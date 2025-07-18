// src/db/crud/account/user_preferences.rs
use crate::db::error::DbError;
use crate::models::preferences::{UserPreferences, UserPreferencesUpdate};
use sqlx::PgPool;

pub async fn find_by_user_id(db: &PgPool, user_id: &str) -> Result<UserPreferences, DbError> {
    let preferences = sqlx::query_as!(
        UserPreferences,
        r#"
        SELECT * FROM user_preferences
        WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_one(db)
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
        INSERT INTO user_preferences (
            user_id,
            auto_subscribe,
            email_notifications,
            push_notifications,
            theme,
            language
        )
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (user_id)
        DO UPDATE SET
            auto_subscribe = COALESCE($2, user_preferences.auto_subscribe),
            email_notifications = COALESCE($3, user_preferences.email_notifications),
            push_notifications = COALESCE($4, user_preferences.push_notifications),
            theme = COALESCE($5, user_preferences.theme),
            language = COALESCE($6, user_preferences.language)
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
    // Try to fetch existing prefs first
    match find_by_user_id(db, user_id).await {
        Ok(prefs) => Ok(prefs),
        Err(DbError::NotFound(_)) => {
            // Create with defaults if doesn't exist
            sqlx::query!(
                r#"
                INSERT INTO user_preferences (user_id)
                VALUES ($1)
                "#,
                user_id
            )
            .execute(db)
            .await?;

            find_by_user_id(db, user_id).await
        }
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::preferences::UserPreferencesUpdate;
    use crate::tests::create_test_user;
    use sqlx::PgPool;

    #[sqlx::test]
    async fn test_upsert_new_preferences(db: PgPool) {
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;

        let update = UserPreferencesUpdate {
            auto_subscribe: Some(true),
            email_notifications: Some(false),
            push_notifications: Some(true),
            theme: Some("dark".to_string()),
            language: Some("es".to_string()),
        };

        let result = upsert(&db, &user_id, &update).await;
        assert!(result.is_ok());

        let prefs = find_by_user_id(&db, &user_id).await.unwrap();
        assert_eq!(prefs.auto_subscribe, true);
        assert_eq!(prefs.email_notifications, false);
        assert_eq!(prefs.theme, "dark");
        assert_eq!(prefs.language, "es");
    }

    #[sqlx::test]
    async fn test_get_or_create_defaults(db: PgPool) {
        let user_id = create_test_user(&db, "newuser", "new@example.com").await;

        let prefs = get_or_create_defaults(&db, &user_id).await.unwrap();

        // Should have default values
        assert_eq!(prefs.auto_subscribe, false);
        assert_eq!(prefs.email_notifications, true);
        assert_eq!(prefs.theme, "system");
        assert_eq!(prefs.language, "en");
    }

    #[sqlx::test]
    async fn test_partial_update(db: PgPool) {
        let user_id = create_test_user(&db, "updateuser", "update@example.com").await;

        // Create with defaults first
        get_or_create_defaults(&db, &user_id).await.unwrap();

        // Partial update - only change auto_subscribe
        let update = UserPreferencesUpdate {
            auto_subscribe: Some(true),
            email_notifications: None,
            push_notifications: None,
            theme: None,
            language: None,
        };

        upsert(&db, &user_id, &update).await.unwrap();

        let prefs = find_by_user_id(&db, &user_id).await.unwrap();
        assert_eq!(prefs.auto_subscribe, true);
        // Other fields should remain defaults
        assert_eq!(prefs.email_notifications, true);
        assert_eq!(prefs.theme, "system");
    }
}
