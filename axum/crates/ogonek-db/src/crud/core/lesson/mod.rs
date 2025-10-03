mod create;
mod delete;
mod read;
mod update;
pub use create::*;
pub use delete::*;
pub use read::*;
pub use update::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::create_test_user;

    use ogonek_types::{LessonCreate, LessonUpdate};
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
        let lesson = read_by_id(&db, &result.unwrap(), &assignee).await;
        assert!(lesson.is_ok());
        assert_eq!(lesson.unwrap().assignee, Some(assignee));
    }

    #[sqlx::test]
    async fn test_read_by_id_as_creator(db: PgPool) {
        let user = create_test_user(&db, "test", "test@ogonek.app").await;

        let lesson_create = LessonCreate {
            title: "Find Test".to_string(),
            topic: "Testing".to_string(),
            markdown: "# Test Content".to_string(),
            assignee: None,
        };

        let creation_id = create(&db, &user, lesson_create).await.unwrap();

        let result = read_by_id(&db, &creation_id, &user).await;
        assert!(result.is_ok());

        let lesson = result.unwrap();
        assert_eq!(lesson.title, "Find Test");
        assert_eq!(lesson.topic, "Testing");
    }

    #[sqlx::test]
    async fn test_read_by_id_as_assignee(db: PgPool) {
        let creator = create_test_user(&db, "creator", "creator@ogonek.app").await;
        let assignee = create_test_user(&db, "assignee", "assignee@ogonek.app").await;

        let lesson_create = LessonCreate {
            title: "Assignee Test".to_string(),
            topic: "Access Control".to_string(),
            markdown: "# Assignee Content".to_string(),
            assignee: Some(assignee.clone()),
        };

        let creation_id = create(&db, &creator, lesson_create).await.unwrap();

        let result = read_by_id(&db, &creation_id, &assignee).await;
        assert!(result.is_ok());

        let lesson = result.unwrap();
        assert_eq!(lesson.assignee, Some(assignee));
        assert_eq!(lesson.assignee_name, Some("Test Name".to_string()));
    }

    #[sqlx::test]
    async fn test_read_by_id_unauthorized(db: PgPool) {
        let creator = create_test_user(&db, "creator", "creator@ogonek.app").await;
        let other_user = create_test_user(&db, "other", "other@ogonek.app").await;

        let lesson_create = LessonCreate {
            title: "Private Lesson".to_string(),
            topic: "Security".to_string(),
            markdown: "# Private Content".to_string(),
            assignee: None,
        };

        let creation_id = create(&db, &creator, lesson_create).await.unwrap();

        let result = read_by_id(&db, &creation_id, &other_user).await;
        assert!(result.is_err());
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
            unassign: None,
        };

        let result = update(&db, &creation_id, &user, &lesson_update).await;
        assert!(result.is_ok());

        // Verify the update
        let updated_lesson = read_by_id(&db, &creation_id, &user).await.unwrap();
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
            unassign: None,
        };

        let result = update(&db, &creation_id, &other_user, &lesson_update).await;
        assert!(result.is_ok()); // Query succeeds but affects 0 rows

        // Verify no changes were made
        let lesson = read_by_id(&db, &creation_id, &creator).await.unwrap();
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
        let read_result = read_by_id(&db, &creation_id, &user).await;
        assert!(read_result.is_err());
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
        let read_result = read_by_id(&db, &creation_id, &creator).await;
        assert!(read_result.is_ok());
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
}
