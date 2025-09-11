use crate::db::error::DbError;
use crate::types::{User, UserRole, UserUpdate};
use sqlx::PgPool;

pub async fn find_by_id(db: &PgPool, user_id: &str) -> Result<User, DbError> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT username, email, role as "role!: UserRole", id, name, pass
        FROM "user"
        WHERE id = $1
        "#,
        user_id
    )
    .fetch_one(db)
    .await?;

    Ok(user)
}

pub async fn delete(db: &PgPool, user_id: &str) -> Result<(), DbError> {
    sqlx::query_as!(
        User,
        r#"
        DELETE FROM "user"
        WHERE id = $1
        "#,
        user_id
    )
    .execute(db)
    .await?;

    Ok(())
}

pub async fn update(db: &PgPool, user_id: &str, update: &UserUpdate) -> Result<(), DbError> {
    sqlx::query_as!(
        User,
        r#"
        UPDATE "user"
        SET
            name = COALESCE($1, name),
            username = COALESCE($2, username),
            email = COALESCE($3, email),
            pass = COALESCE($4, pass)
        WHERE id = $5
        "#,
        update.name,
        update.username,
        update.email,
        update.pass,
        user_id
    )
    .execute(db)
    .await
    .map_err(|e| {
        if let sqlx::Error::Database(dbe) = &e
            && let Some(constraint) = dbe.constraint()
        {
            if constraint == "user_username_key" {
                return DbError::AlreadyExists("Username already taken".into());
            }
            if constraint == "user_email_key" {
                return DbError::AlreadyExists("Email already taken".into());
            }
        }
        tracing::error!("Database error when updating user: {:?}", e);
        DbError::Database(e)
    })?;

    Ok(())
}

pub async fn get_email(db: &PgPool, user_id: &str) -> Result<String, DbError> {
    let email = sqlx::query_scalar!(
        r#"
        SELECT email
        FROM "user"
        WHERE id = $1
        "#,
        user_id
    )
    .fetch_one(db)
    .await?;

    Ok(email)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::error::DbError;
    use crate::tests::{cleanup_user, create_test_user};
    use crate::types::UserUpdate;
    use sqlx::PgPool;

    #[sqlx::test]
    async fn test_find_by_id_success(db: PgPool) {
        // Arrange
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;

        // Act
        let result = find_by_id(&db, &user_id).await;

        // Assert
        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.id, user_id);
        assert_eq!(user.username, "testuser");
        assert_eq!(user.email, "test@example.com");
        assert_eq!(user.name, "Test Name");

        // Cleanup
        cleanup_user(&db, &user_id).await;
    }

    #[sqlx::test]
    async fn test_find_by_id_not_found(db: PgPool) {
        // Arrange
        let non_existent_id = nanoid::nanoid!();

        // Act
        let result = find_by_id(&db, &non_existent_id).await;

        // Assert
        assert!(result.is_err());
    }

    #[sqlx::test]
    async fn test_find_by_id_invalid_uuid_format(db: PgPool) {
        // Act
        let result = find_by_id(&db, "invalid-uuid").await;

        // Assert
        assert!(result.is_err());
    }

    #[sqlx::test]
    async fn test_delete_success(db: PgPool) {
        // Arrange
        let user_id = create_test_user(&db, "deleteuser", "delete@example.com").await;

        // Verify user exists first
        let user_exists = find_by_id(&db, &user_id).await;
        assert!(user_exists.is_ok());

        // Act
        let result = delete(&db, &user_id).await;

        // Assert
        assert!(result.is_ok());

        // Verify user is actually deleted
        let user_gone = find_by_id(&db, &user_id).await;
        assert!(user_gone.is_err());
    }

    #[sqlx::test]
    async fn test_delete_non_existent_user(db: PgPool) {
        // Arrange
        let non_existent_id = nanoid::nanoid!();

        // Act
        let result = delete(&db, &non_existent_id).await;

        // Assert
        // Delete should succeed even if user doesn't exist (no rows affected)
        assert!(result.is_ok());
    }

    #[sqlx::test]
    async fn test_update_success_partial_update(db: PgPool) {
        // Arrange
        let user_id = create_test_user(&db, "updateuser", "update@example.com").await;
        let upd = UserUpdate {
            name: Some("Updated Name".to_string()),
            username: None,
            email: None,
            pass: None,
        };

        // Act
        let result = update(&db, &user_id, &upd).await;

        // Assert
        assert!(result.is_ok());

        // Verify the update
        let updated_user = find_by_id(&db, &user_id).await.unwrap();
        assert_eq!(updated_user.name, "Updated Name");
        assert_eq!(updated_user.username, "updateuser"); // Should remain unchanged
        assert_eq!(updated_user.email, "update@example.com"); // Should remain unchanged

        // Cleanup
        cleanup_user(&db, &user_id).await;
    }

    #[sqlx::test]
    async fn test_update_success_full_update(db: PgPool) {
        // Arrange
        let user_id = create_test_user(&db, "fullupdate", "full@example.com").await;
        let upd = UserUpdate {
            name: Some("New Name".to_string()),
            username: Some("newusername".to_string()),
            email: Some("new@example.com".to_string()),
            pass: Some("new_hashed_password".to_string()),
        };

        // Act
        let result = update(&db, &user_id, &upd).await;

        // Assert
        assert!(result.is_ok());

        // Verify the update
        let updated_user = find_by_id(&db, &user_id).await.unwrap();
        assert_eq!(updated_user.name, "New Name");
        assert_eq!(updated_user.username, "newusername");
        assert_eq!(updated_user.email, "new@example.com");
        assert_eq!(updated_user.pass, "new_hashed_password");

        // Cleanup
        cleanup_user(&db, &user_id).await;
    }

    #[sqlx::test]
    async fn test_update_no_changes(db: PgPool) {
        // Arrange
        let user_id = create_test_user(&db, "nochange", "nochange@example.com").await;
        let original_user = find_by_id(&db, &user_id).await.unwrap();

        let upd = UserUpdate {
            name: None,
            username: None,
            email: None,
            pass: None,
        };

        // Act
        let result = update(&db, &user_id, &upd).await;

        // Assert
        assert!(result.is_ok());

        // Verify nothing changed
        let unchanged_user = find_by_id(&db, &user_id).await.unwrap();
        assert_eq!(unchanged_user.name, original_user.name);
        assert_eq!(unchanged_user.username, original_user.username);
        assert_eq!(unchanged_user.email, original_user.email);
        assert_eq!(unchanged_user.pass, original_user.pass);
        assert_eq!(unchanged_user.role, original_user.role);

        // Cleanup
        cleanup_user(&db, &user_id).await;
    }

    #[sqlx::test]
    async fn test_update_duplicate_username_error(db: PgPool) {
        // Arrange
        let user1_id = create_test_user(&db, "user1", "user1@example.com").await;
        let user2_id = create_test_user(&db, "user2", "user2@example.com").await;

        let upd = UserUpdate {
            name: None,
            username: Some("user1".to_string()), // Try to use existing username
            email: None,
            pass: None,
        };

        // Act
        let result = update(&db, &user2_id, &upd).await;

        // Assert
        assert!(result.is_err());
        match result.unwrap_err() {
            DbError::AlreadyExists(msg) => {
                assert_eq!(msg, "Username already taken");
            }
            other => panic!("Expected AlreadyExists error, got: {:?}", other),
        }

        // Cleanup
        cleanup_user(&db, &user1_id).await;
        cleanup_user(&db, &user2_id).await;
    }

    #[sqlx::test]
    async fn test_update_duplicate_email_error(db: PgPool) {
        // Arrange
        let user1_id = create_test_user(&db, "emailuser1", "duplicate@example.com").await;
        let user2_id = create_test_user(&db, "emailuser2", "unique@example.com").await;

        let upd = UserUpdate {
            name: None,
            username: None,
            email: Some("duplicate@example.com".to_string()), // Try to use existing email
            pass: None,
        };

        // Act
        let result = update(&db, &user2_id, &upd).await;

        // Assert
        assert!(result.is_err());
        match result.unwrap_err() {
            DbError::AlreadyExists(msg) => {
                assert_eq!(msg, "Email already taken");
            }
            other => panic!("Expected AlreadyExists error, got: {:?}", other),
        }

        // Cleanup
        cleanup_user(&db, &user1_id).await;
        cleanup_user(&db, &user2_id).await;
    }

    #[sqlx::test]
    async fn test_update_non_existent_user(db: PgPool) {
        // Arrange
        let non_existent_id = nanoid::nanoid!();
        let upd = UserUpdate {
            name: Some("New Name".to_string()),
            username: None,
            email: None,
            pass: None,
        };

        // Act
        let result = update(&db, &non_existent_id, &upd).await;

        // Assert
        // Update should succeed even if no rows are affected
        assert!(result.is_ok());
    }

    #[sqlx::test]
    async fn test_concurrent_operations(db: PgPool) {
        // Arrange
        let user_id = create_test_user(&db, "concurrent", "concurrent@example.com").await;

        // Act - Simulate concurrent operations
        let upd1 = UserUpdate {
            name: Some("Name 1".to_string()),
            username: None,
            email: None,
            pass: None,
        };

        let upd2 = UserUpdate {
            name: Some("Name 2".to_string()),
            username: None,
            email: None,
            pass: None,
        };

        let (result1, result2) =
            tokio::join!(update(&db, &user_id, &upd1), update(&db, &user_id, &upd2));

        // Assert
        assert!(result1.is_ok());
        assert!(result2.is_ok());

        // One of the updates should have taken effect
        let final_user = find_by_id(&db, &user_id).await.unwrap();
        assert!(final_user.name == "Name 1" || final_user.name == "Name 2");

        // Cleanup
        cleanup_user(&db, &user_id).await;
    }
}
