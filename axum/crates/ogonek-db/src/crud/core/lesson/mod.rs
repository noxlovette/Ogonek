use crate::DbError;
use ogonek_types::{LessonCreate, LessonUpdate};
use sqlx::PgPool;

mod read;
pub use read::*;
pub async fn create(db: &PgPool, user_id: &str, create: LessonCreate) -> Result<String, DbError> {
    let mut assignee = user_id;

    if create.assignee.is_some() {
        assignee = create.assignee.as_ref().unwrap();
    }

    let id = sqlx::query_scalar!(
        "INSERT INTO lessons (id, title, topic, markdown, created_by, assignee)
         VALUES ($1, $2, $3, $4, $5, $6)
         RETURNING id",
        nanoid::nanoid!(),
        create.title,
        create.topic,
        create.markdown,
        user_id,
        assignee
    )
    .fetch_one(db)
    .await?;

    Ok(id)
}

/// Takes user preferences to define defaults (well, it currently doesn't but you get the point)
pub async fn create_with_defaults(db: &PgPool, user_id: &str) -> Result<String, DbError> {
    let id = sqlx::query_scalar!(
        "INSERT INTO lessons (id, title, topic, markdown, created_by, assignee)
         VALUES ($1, $2, $3, $4, $5, $6)
         RETURNING id",
        nanoid::nanoid!(),
        "Default Title",
        "Default Topic",
        "# Default Lesson",
        user_id,
        user_id
    )
    .fetch_one(db)
    .await?;

    Ok(id)
}

pub async fn delete(db: &PgPool, lesson_id: &str, user_id: &str) -> Result<(), DbError> {
    sqlx::query!(
        r#"
        DELETE FROM lessons
        WHERE id = $1 AND created_by = $2
        "#,
        lesson_id,
        user_id
    )
    .execute(db)
    .await?;

    Ok(())
}

pub async fn update(
    db: &PgPool,
    lesson_id: &str,
    user_id: &str,
    update: &LessonUpdate,
) -> Result<(), DbError> {
    sqlx::query!(
        "UPDATE lessons
         SET
            title = COALESCE($1, title),
            topic = COALESCE($2, topic),
            markdown = COALESCE($3, markdown),
            assignee = COALESCE($4, assignee)
         WHERE id = $5 AND created_by = $6
",
        update.title,
        update.topic,
        update.markdown,
        update.assignee,
        lesson_id,
        user_id
    )
    .execute(db)
    .await?;

    Ok(())
}

/// Finds assignee for the lesson by its id, will return null if the user doesn't have access to the data
pub async fn read_assignee(
    db: &PgPool,
    lesson_id: &str,
    user_id: &str,
) -> Result<Option<String>, DbError> {
    let assignee = sqlx::query_scalar!(
        r#"
        SELECT assignee
        FROM lessons
        WHERE id = $1
        AND (assignee = $2 OR created_by = $2)
        "#,
        lesson_id,
        user_id
    )
    .fetch_optional(db) // in case lesson is not found
    .await?;

    Ok(assignee)
}

pub async fn count(db: &PgPool, user_id: &str) -> Result<i64, DbError> {
    let count = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM lessons WHERE
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
    use crate::tests::create_test_user;

    use ogonek_types::{LessonCreate, LessonUpdate, PaginationParams};
    use sqlx::PgPool;

    #[sqlx::test]
    async fn test_create_lesson(db: PgPool) {
        let user = create_test_user(&db, "test", "test@ogonek.app").await;

        let lesson_create = LessonCreate {
            title: "Test Lesson".to_string(),
            topic: "Rust Programming".to_string(),
            markdown: "# Introduction\nThis is a test lesson.".to_string(),
            assignee: None,
        };

        let result = create(&db, &user, lesson_create).await;
        assert!(result.is_ok());

        let creation_id = result.unwrap();
        assert!(!creation_id.is_empty());
    }

    #[sqlx::test]
    async fn test_create_lesson_succes_with_defaults(db: PgPool) {
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;

        let result = create_with_defaults(&db, &user_id).await;
        assert!(result.is_ok());

        let lesson_id = result.unwrap();
        assert!(!lesson_id.is_empty());
    }

    #[sqlx::test]
    async fn test_create_lesson_with_assignee(db: PgPool) {
        let creator = create_test_user(&db, "creator", "creator@ogonek.app").await;
        let assignee = create_test_user(&db, "assignee", "assignee@ogonek.app").await;

        let lesson_create = LessonCreate {
            title: "Assigned Lesson".to_string(),
            topic: "Database Design".to_string(),
            markdown: "# Database Basics\nLearn about databases.".to_string(),
            assignee: Some(assignee.clone()),
        };

        let result = create(&db, &creator, lesson_create).await;
        assert!(result.is_ok());

        // Verify the lesson was created with correct assignee
        let lesson = find_by_id(&db, &result.unwrap(), &assignee).await;
        assert!(lesson.is_ok());
        assert_eq!(lesson.unwrap().assignee, assignee);
    }

    #[sqlx::test]
    async fn test_find_by_id_as_creator(db: PgPool) {
        let user = create_test_user(&db, "test", "test@ogonek.app").await;

        let lesson_create = LessonCreate {
            title: "Find Test".to_string(),
            topic: "Testing".to_string(),
            markdown: "# Test Content".to_string(),
            assignee: None,
        };

        let creation_id = create(&db, &user, lesson_create).await.unwrap();

        let result = find_by_id(&db, &creation_id, &user).await;
        assert!(result.is_ok());

        let lesson = result.unwrap();
        assert_eq!(lesson.title, "Find Test");
        assert_eq!(lesson.topic, "Testing");
    }

    #[sqlx::test]
    async fn test_find_by_id_as_assignee(db: PgPool) {
        let creator = create_test_user(&db, "creator", "creator@ogonek.app").await;
        let assignee = create_test_user(&db, "assignee", "assignee@ogonek.app").await;

        let lesson_create = LessonCreate {
            title: "Assignee Test".to_string(),
            topic: "Access Control".to_string(),
            markdown: "# Assignee Content".to_string(),
            assignee: Some(assignee.clone()),
        };

        let creation_id = create(&db, &creator, lesson_create).await.unwrap();

        let result = find_by_id(&db, &creation_id, &assignee).await;
        assert!(result.is_ok());

        let lesson = result.unwrap();
        assert_eq!(lesson.assignee, assignee);
        assert_eq!(lesson.assignee_name, "Test Name");
    }

    #[sqlx::test]
    async fn test_find_by_id_unauthorized(db: PgPool) {
        let creator = create_test_user(&db, "creator", "creator@ogonek.app").await;
        let other_user = create_test_user(&db, "other", "other@ogonek.app").await;

        let lesson_create = LessonCreate {
            title: "Private Lesson".to_string(),
            topic: "Security".to_string(),
            markdown: "# Private Content".to_string(),
            assignee: None,
        };

        let creation_id = create(&db, &creator, lesson_create).await.unwrap();

        let result = find_by_id(&db, &creation_id, &other_user).await;
        assert!(result.is_err());
    }

    #[sqlx::test]
    async fn test_find_all_basic(db: PgPool) {
        let user = create_test_user(&db, "test", "test@ogonek.app").await;

        // Create multiple lessons
        for i in 1..=3 {
            let lesson_create = LessonCreate {
                title: format!("Lesson {}", i),
                topic: format!("Topic {}", i),
                markdown: format!("# Content {}", i),
                assignee: None,
            };
            create(&db, &user, lesson_create).await.unwrap();
        }

        let params = PaginationParams {
            page: Some(1),
            per_page: Some(10),
            search: None,
            assignee: None,
        };

        let result = find_all(&db, &user, &params).await;
        assert!(result.is_ok());

        let lessons = result.unwrap();
        assert_eq!(lessons.len(), 3);
    }

    #[sqlx::test]
    async fn test_find_all_with_search(db: PgPool) {
        let user = create_test_user(&db, "test", "test@ogonek.app").await;

        let lesson_create1 = LessonCreate {
            title: "Rust Programming".to_string(),
            topic: "Programming".to_string(),
            markdown: "# Rust Basics".to_string(),
            assignee: None,
        };

        let lesson_create2 = LessonCreate {
            title: "Python Basics".to_string(),
            topic: "Programming".to_string(),
            markdown: "# Python Introduction".to_string(),
            assignee: None,
        };

        create(&db, &user, lesson_create1).await.unwrap();
        create(&db, &user, lesson_create2).await.unwrap();

        let params = PaginationParams {
            page: Some(1),
            per_page: Some(10),
            search: Some("Rust".to_string()),
            assignee: None,
        };

        let result = find_all(&db, &user, &params).await;
        assert!(result.is_ok());

        let lessons = result.unwrap();
        assert_eq!(lessons.len(), 1);
        assert!(lessons[0].title.contains("Rust"));
    }

    #[sqlx::test]
    async fn test_find_all_with_assignee_filter(db: PgPool) {
        let creator = create_test_user(&db, "creator", "creator@ogonek.app").await;
        let assignee = create_test_user(&db, "assignee", "assignee@ogonek.app").await;

        let lesson_create1 = LessonCreate {
            title: "Self Assigned".to_string(),
            topic: "Self Study".to_string(),
            markdown: "# Self Content".to_string(),
            assignee: None,
        };

        let lesson_create2 = LessonCreate {
            title: "Other Assigned".to_string(),
            topic: "Collaboration".to_string(),
            markdown: "# Shared Content".to_string(),
            assignee: Some(assignee.clone()),
        };

        create(&db, &creator, lesson_create1).await.unwrap();
        create(&db, &creator, lesson_create2).await.unwrap();

        let params = PaginationParams {
            page: Some(1),
            per_page: Some(10),
            search: None,
            assignee: Some(assignee.clone()),
        };

        let result = find_all(&db, &creator, &params).await;
        assert!(result.is_ok());

        let lessons = result.unwrap();
        assert_eq!(lessons.len(), 1);
    }

    #[sqlx::test]
    async fn test_find_recent(db: PgPool) {
        let user = create_test_user(&db, "test", "test@ogonek.app").await;

        // Create more than 6 lessons to test the limit
        for i in 1..=8 {
            let lesson_create = LessonCreate {
                title: format!("Recent Lesson {}", i),
                topic: "Recent Topic".to_string(),
                markdown: format!(
                    "# Recent Content {} with some longer text to test truncation",
                    i
                ),
                assignee: None,
            };
            create(&db, &user, lesson_create).await.unwrap();
        }

        let result = find_recent(&db, &user).await;
        assert!(result.is_ok());

        let lessons = result.unwrap();
        assert_eq!(lessons.len(), 3); // Should be limited to 6
    }

    #[sqlx::test]
    async fn test_update_lesson(db: PgPool) {
        let user = create_test_user(&db, "test", "test@ogonek.app").await;

        let lesson_create = LessonCreate {
            title: "Original Title".to_string(),
            topic: "Original Topic".to_string(),
            markdown: "# Original Content".to_string(),
            assignee: None,
        };

        let creation_id = create(&db, &user, lesson_create).await.unwrap();

        let lesson_update = LessonUpdate {
            title: Some("Updated Title".to_string()),
            topic: Some("Updated Topic".to_string()),
            markdown: None,
            assignee: None,
            media_url: None,
            id: None,
            created_by: None,
        };

        let result = update(&db, &creation_id, &user, &lesson_update).await;
        assert!(result.is_ok());

        // Verify the update
        let updated_lesson = find_by_id(&db, &creation_id, &user).await.unwrap();
        assert_eq!(updated_lesson.title, "Updated Title");
        assert_eq!(updated_lesson.topic, "Updated Topic");
        assert_eq!(updated_lesson.markdown, "# Original Content"); // Should remain unchanged
    }

    #[sqlx::test]
    async fn test_update_lesson_unauthorized(db: PgPool) {
        let creator = create_test_user(&db, "creator", "creator@ogonek.app").await;
        let other_user = create_test_user(&db, "other", "other@ogonek.app").await;

        let lesson_create = LessonCreate {
            title: "Private Lesson".to_string(),
            topic: "Private Topic".to_string(),
            markdown: "# Private Content".to_string(),
            assignee: None,
        };

        let creation_id = create(&db, &creator, lesson_create).await.unwrap();

        let lesson_update = LessonUpdate {
            title: Some("Hacked Title".to_string()),
            topic: None,
            markdown: None,
            assignee: None,
            media_url: None,
            id: None,
            created_by: None,
        };

        let result = update(&db, &creation_id, &other_user, &lesson_update).await;
        assert!(result.is_ok()); // Query succeeds but affects 0 rows

        // Verify no changes were made
        let lesson = find_by_id(&db, &creation_id, &creator).await.unwrap();
        assert_eq!(lesson.title, "Private Lesson");
    }

    #[sqlx::test]
    async fn test_delete_lesson(db: PgPool) {
        let user = create_test_user(&db, "test", "test@ogonek.app").await;

        let lesson_create = LessonCreate {
            title: "To Delete".to_string(),
            topic: "Deletion".to_string(),
            markdown: "# Will be deleted".to_string(),
            assignee: None,
        };

        let creation_id = create(&db, &user, lesson_create).await.unwrap();

        let delete_result = delete(&db, &creation_id, &user).await;
        assert!(delete_result.is_ok());

        // Verify lesson is deleted
        let find_result = find_by_id(&db, &creation_id, &user).await;
        assert!(find_result.is_err());
    }

    #[sqlx::test]
    async fn test_delete_lesson_unauthorized(db: PgPool) {
        let creator = create_test_user(&db, "creator", "creator@ogonek.app").await;
        let other_user = create_test_user(&db, "assignee", "assignee@ogonek.app").await;

        let lesson_create = LessonCreate {
            title: "Protected Lesson".to_string(),
            topic: "Security".to_string(),
            markdown: "# Protected Content".to_string(),
            assignee: None,
        };

        let creation_id = create(&db, &creator, lesson_create).await.unwrap();

        let delete_result = delete(&db, &creation_id, &other_user).await;
        assert!(delete_result.is_ok()); // Query succeeds but affects 0 rows

        // Verify lesson still exists
        let find_result = find_by_id(&db, &creation_id, &creator).await;
        assert!(find_result.is_ok());
    }

    #[sqlx::test]
    async fn test_count_lessons(db: PgPool) {
        let user = create_test_user(&db, "creator", "creator@ogonek.app").await;
        let assignee = create_test_user(&db, "assignee", "assignee@ogonek.app").await;

        // Initial count should be 0
        let initial_count = count(&db, &user).await.unwrap();
        assert_eq!(initial_count, 0);

        // Create lessons as creator
        for i in 1..=3 {
            let lesson_create = LessonCreate {
                title: format!("Creator Lesson {}", i),
                topic: "Creator Topic".to_string(),
                markdown: "# Creator Content".to_string(),
                assignee: None,
            };
            create(&db, &user, lesson_create).await.unwrap();
        }

        // Create lesson assigned to user
        let assigned_lesson = LessonCreate {
            title: "Assigned Lesson".to_string(),
            topic: "Assignment".to_string(),
            markdown: "# Assigned Content".to_string(),
            assignee: Some(user.clone()),
        };
        create(&db, &assignee, assigned_lesson).await.unwrap();

        let final_count = count(&db, &user).await.unwrap();
        assert_eq!(final_count, 4); // 3 created + 1 assigned
    }

    #[sqlx::test]
    async fn test_pagination_params(db: PgPool) {
        let user = create_test_user(&db, "test", "test@ogonek.app").await;

        // Create 15 lessons
        for i in 1..=15 {
            let lesson_create = LessonCreate {
                title: format!("Pagination Lesson {}", i),
                topic: "Pagination".to_string(),
                markdown: format!("# Content {}", i),
                assignee: None,
            };
            create(&db, &user, lesson_create).await.unwrap();
        }

        // Test first page
        let params1 = PaginationParams {
            page: Some(1),
            per_page: Some(5),
            search: None,
            assignee: None,
        };

        let result1 = find_all(&db, &user, &params1).await.unwrap();
        assert_eq!(result1.len(), 5);

        // Test second page
        let params2 = PaginationParams {
            page: Some(2),
            per_page: Some(5),
            search: None,
            assignee: None,
        };

        let result2 = find_all(&db, &user, &params2).await.unwrap();
        assert_eq!(result2.len(), 5);

        // Test third page
        let params3 = PaginationParams {
            page: Some(3),
            per_page: Some(5),
            search: None,
            assignee: None,
        };

        let result3 = find_all(&db, &user, &params3).await.unwrap();
        assert_eq!(result3.len(), 5);

        // Test fourth page (should have no results)
        let params4 = PaginationParams {
            page: Some(4),
            per_page: Some(5),
            search: None,
            assignee: None,
        };

        let result4 = find_all(&db, &user, &params4).await.unwrap();
        assert_eq!(result4.len(), 0);
    }
}
