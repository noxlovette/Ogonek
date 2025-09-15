use sqlx::PgPool;

use crate::{
    db::error::DbError,
    types::{Calendar, CalendarCreate, CalendarUpdate},
};
/// Finds the calendar by id
pub async fn find_by_id(
    db: &PgPool,
    calendar_id: &str,
    user_id: &str,
) -> Result<Calendar, DbError> {
    let calendar = sqlx::query_as!(
        Calendar,
        r#"
      SELECT *
      FROM calendars
      WHERE id = $1 AND owner_id = $2
      "#,
        calendar_id,
        user_id
    )
    .fetch_one(db)
    .await?;

    Ok(calendar)
}

/// Finds all calendars for the user
pub async fn find_all(db: &PgPool, user_id: &str) -> Result<Vec<Calendar>, DbError> {
    let calendars = sqlx::query_as!(
        Calendar,
        r#"
      SELECT *
      FROM calendars
      WHERE owner_id = $1
      "#,
        user_id
    )
    .fetch_all(db)
    .await?;

    Ok(calendars)
}

/// Deletes a cal
pub async fn delete(db: &PgPool, calendar_id: &str, user_id: &str) -> Result<(), DbError> {
    sqlx::query!(
        r#"
    DELETE
      FROM calendars
      WHERE owner_id = $2 AND id = $1
      "#,
        calendar_id,
        user_id
    )
    .execute(db)
    .await?;

    Ok(())
}

/// Updates the calendar in the DB
pub async fn update(
    db: &PgPool,
    calendar_id: &str,
    user_id: &str,
    update: &CalendarUpdate,
) -> Result<(), DbError> {
    sqlx::query!(
        "UPDATE calendars 
         SET
            name = COALESCE($1, name),
            description = COALESCE($2, description),
            colour = COALESCE($3, colour),
            timezone = COALESCE($4, timezone),
            caldav_url = COALESCE($5, caldav_url),
            sync_token = COALESCE($6, sync_token)
         WHERE id = $7 AND owner_id = $8
",
        update.name,
        update.description,
        update.colour,
        update.timezone,
        update.caldav_url,
        update.sync_token,
        calendar_id,
        user_id
    )
    .execute(db)
    .await?;

    Ok(())
}

/// Creates a calendar entry. The initial request should contain the name for it
pub async fn create(db: &PgPool, user_id: &str, create: CalendarCreate) -> Result<String, DbError> {
    let id = sqlx::query_scalar!(
        "INSERT INTO calendars (id, name, owner_id)
         VALUES ($1, $2, $3)
         RETURNING id",
        nanoid::nanoid!(),
        create.name,
        user_id,
    )
    .fetch_one(db)
    .await?;

    Ok(id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::create_test_user;
    use crate::types::CalendarCreate;

    async fn create_test_calendar(db: &PgPool, user_id: &str, name: &str) -> String {
        let calendar_create = CalendarCreate {
            name: name.to_string(),
        };

        create(db, user_id, calendar_create).await.unwrap()
    }

    #[sqlx::test]
    async fn test_create_calendar(db: PgPool) {
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;

        let calendar_create = CalendarCreate {
            name: "My Test Calendar".to_string(),
        };

        let result = create(&db, &user_id, calendar_create).await;
        assert!(result.is_ok());

        let calendar_id = result.unwrap();
        assert!(!calendar_id.is_empty());
    }

    #[sqlx::test]
    async fn test_find_by_id_success(db: PgPool) {
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let calendar_id = create_test_calendar(&db, &user_id, "Test Calendar").await;

        let result = find_by_id(&db, &calendar_id, &user_id).await;
        assert!(result.is_ok());

        let calendar = result.unwrap();
        assert_eq!(calendar.id, calendar_id);
        assert_eq!(calendar.name, "Test Calendar");
        assert_eq!(calendar.owner_id, user_id);
    }

    #[sqlx::test]
    async fn test_find_by_id_not_found(db: PgPool) {
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let non_existent_id = "non-existent-id";

        let result = find_by_id(&db, non_existent_id, &user_id).await;
        assert!(result.is_err());
    }

    #[sqlx::test]
    async fn test_find_by_id_unauthorized(db: PgPool) {
        let owner_id = create_test_user(&db, "owner", "owner@example.com").await;
        let other_user_id = create_test_user(&db, "other", "other@example.com").await;
        let calendar_id = create_test_calendar(&db, &owner_id, "Owner's Calendar").await;

        let result = find_by_id(&db, &calendar_id, &other_user_id).await;
        assert!(result.is_err());
    }

    #[sqlx::test]
    async fn test_find_all_basic(db: PgPool) {
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;

        // Create multiple calendars
        create_test_calendar(&db, &user_id, "Calendar 1").await;
        create_test_calendar(&db, &user_id, "Calendar 2").await;
        create_test_calendar(&db, &user_id, "Calendar 3").await;

        let result = find_all(&db, &user_id).await;
        assert!(result.is_ok());

        let calendars = result.unwrap();
        assert_eq!(calendars.len(), 3);
    }

    #[sqlx::test]
    async fn test_find_all_empty_result(db: PgPool) {
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;

        let result = find_all(&db, &user_id).await;
        assert!(result.is_ok());

        let calendars = result.unwrap();
        assert_eq!(calendars.len(), 0);
    }

    #[sqlx::test]
    async fn test_update_calendar(db: PgPool) {
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let calendar_id = create_test_calendar(&db, &user_id, "Original Name").await;

        let update_data = CalendarUpdate {
            name: Some("Updated Name".to_string()),
            description: Some("Updated description".to_string()),
            colour: Some("#FF5733".to_string()),
            timezone: Some("America/New_York".to_string()),
            caldav_url: Some("https://example.com/caldav".to_string()),
            sync_token: Some("token123".to_string()),
        };

        let result = update(&db, &calendar_id, &user_id, &update_data).await;
        assert!(result.is_ok());

        // Verify the update
        let calendar = find_by_id(&db, &calendar_id, &user_id).await.unwrap();
        assert_eq!(calendar.name, "Updated Name");
        assert_eq!(calendar.description, Some("Updated description".to_string()));
        assert_eq!(calendar.colour, "#FF5733");
    }

    #[sqlx::test]
    async fn test_update_partial(db: PgPool) {
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let calendar_id = create_test_calendar(&db, &user_id, "Original Name").await;

        let update_data = CalendarUpdate {
            name: Some("Updated Name".to_string()),
            description: None,
            colour: None,
            timezone: None,
            caldav_url: None,
            sync_token: None,
        };

        let result = update(&db, &calendar_id, &user_id, &update_data).await;
        assert!(result.is_ok());

        // Verify only name was updated
        let calendar = find_by_id(&db, &calendar_id, &user_id).await.unwrap();
        assert_eq!(calendar.name, "Updated Name");
    }

    #[sqlx::test]
    async fn test_update_unauthorized(db: PgPool) {
        let owner_id = create_test_user(&db, "owner", "owner@example.com").await;
        let other_user_id = create_test_user(&db, "other", "other@example.com").await;
        let calendar_id = create_test_calendar(&db, &owner_id, "Owner's Calendar").await;

        let update_data = CalendarUpdate {
            name: Some("Hacked Name".to_string()),
            description: None,
            colour: None,
            timezone: None,
            caldav_url: None,
            sync_token: None,
        };

        let result = update(&db, &calendar_id, &other_user_id, &update_data).await;
        assert!(result.is_ok()); // Update doesn't fail, but affects no rows

        // Verify name wasn't changed
        let calendar = find_by_id(&db, &calendar_id, &owner_id).await.unwrap();
        assert_eq!(calendar.name, "Owner's Calendar");
    }

    #[sqlx::test]
    async fn test_delete_calendar(db: PgPool) {
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let calendar_id = create_test_calendar(&db, &user_id, "Calendar to Delete").await;

        let result = delete(&db, &calendar_id, &user_id).await;
        assert!(result.is_ok());

        // Verify calendar is deleted
        let find_result = find_by_id(&db, &calendar_id, &user_id).await;
        assert!(find_result.is_err());
    }

    #[sqlx::test]
    async fn test_delete_unauthorized(db: PgPool) {
        let owner_id = create_test_user(&db, "owner", "owner@example.com").await;
        let other_user_id = create_test_user(&db, "other", "other@example.com").await;
        let calendar_id = create_test_calendar(&db, &owner_id, "Owner's Calendar").await;

        let result = delete(&db, &calendar_id, &other_user_id).await;
        assert!(result.is_ok()); // Delete doesn't fail, but affects no rows

        // Verify calendar still exists
        let calendar = find_by_id(&db, &calendar_id, &owner_id).await;
        assert!(calendar.is_ok());
    }

    #[sqlx::test]
    async fn test_delete_non_existent(db: PgPool) {
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let non_existent_id = "non-existent-id";

        let result = delete(&db, non_existent_id, &user_id).await;
        assert!(result.is_ok()); // Delete doesn't fail for non-existent records
    }
}
