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

    use crate::{
        crud::core::file::add_files,
        tests::create_test_user,
        types::{TaskCreate, TaskPaginationParams, TaskUpdate},
    };

    use chrono::Utc;
    use sqlx::PgPool;

    // Helper function to create test files
    async fn create_test_file(db: &PgPool, user_id: &str) -> String {
        let file_id = nanoid::nanoid!();

        sqlx::query!(
            r#"
            INSERT INTO files (
                id, name, s3_key, path, mime_type, size, is_folder, parent_id, owner_id, visibility, upload_status
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, NULL, $8, $9, $10)
            "#,
            file_id,                // $1: id
            "name",                 // $2: name
            "s3",                   // $3: s3_key
            "path",                 // $4: path
            "type",                 // $5: mime_type
            100,                    // $6: size
            false,                  // $7: is_folder
            user_id,                // $8: owner_id
            "private",              // $9: visibility
            "complete",             // $10: upload_status
        )
        .execute(db)
        .await
        .expect("Failed to insert test file");

        file_id
    }

    // Helper function to create a basic task
    async fn create_test_task(db: &PgPool, creator_id: &str, assignee_id: &str) -> String {
        let task_create = TaskCreate {
            title: "test task".to_string(),
            markdown: format!("# {}\nTest task content", "test"),
            due_date: Some(Utc::now()),
            priority: Some(1),
            assignee: None,
        };

        let id = create(db, &task_create, creator_id, assignee_id)
            .await
            .unwrap();

        id
    }

    #[sqlx::test]
    async fn test_create_task(db: PgPool) {
        let creator_id = create_test_user(&db, "creator", "creator@test.com").await;
        let assignee_id = create_test_user(&db, "assignee", "assignee@test.com").await;

        let id = create_test_task(&db, &creator_id, &assignee_id).await;

        assert!(!id.is_empty());
    }

    #[sqlx::test]
    async fn test_create_task_succes_with_defaults(db: PgPool) {
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;

        let result = create_with_defaults(&db, &user_id).await;
        assert!(result.is_ok());

        let task_id = result.unwrap();
        assert!(!task_id.is_empty());
    }

    #[sqlx::test]
    async fn test_read_by_id_as_creator(db: PgPool) {
        let creator_id = create_test_user(&db, "creator", "creator@test.com").await;
        let assignee_id = create_test_user(&db, "assignee", "assignee@test.com").await;

        let task_id = create_test_task(&db, &creator_id, &assignee_id).await;

        let result = read_by_id(&db, &task_id, &creator_id).await;
        assert!(result.is_ok());

        let task = result.unwrap();
        assert_eq!(task.title, "test task");
        assert_eq!(task.created_by, creator_id);
        assert_eq!(task.assignee, assignee_id);
    }

    #[sqlx::test]
    async fn test_read_by_id_as_assignee(db: PgPool) {
        let creator_id = create_test_user(&db, "creator", "creator@test.com").await;
        let assignee_id = create_test_user(&db, "assignee", "assignee@test.com").await;

        let task_id = create_test_task(&db, &creator_id, &assignee_id).await;
        let result = read_by_id(&db, &task_id, &assignee_id).await;
        assert!(result.is_ok());

        let task = result.unwrap();
        assert_eq!(task.assignee, assignee_id);
    }

    #[sqlx::test]
    async fn test_read_by_id_unauthorized(db: PgPool) {
        let creator_id = create_test_user(&db, "creator", "creator@test.com").await;
        let assignee_id = create_test_user(&db, "assignee", "assignee@test.com").await;
        let other_user_id = create_test_user(&db, "other", "other@test.com").await;
        let task_id = create_test_task(&db, &creator_id, &assignee_id).await;
        let result = read_by_id(&db, &task_id, &other_user_id).await;
        assert!(result.is_err());
    }

    #[sqlx::test]
    async fn test_read_all_basic(db: PgPool) {
        let user_id = create_test_user(&db, "user", "user@test.com").await;

        // Create multiple tasks
        for _i in 1..=3 {
            create_test_task(&db, &user_id, &user_id).await;
        }

        let params = TaskPaginationParams {
            page: Some(1),
            per_page: Some(10),
            search: None,
            completed: None,
            assignee: None,
        };

        let result = read_all(&db, &user_id, &params).await;
        assert!(result.is_ok());

        let tasks = result.unwrap();
        assert_eq!(tasks.len(), 3);
    }

    #[sqlx::test]
    async fn test_read_all_with_search(db: PgPool) {
        let user_id = create_test_user(&db, "user", "user@test.com").await;

        create_test_task(&db, &user_id, &user_id).await;
        create_test_task(&db, &user_id, &user_id).await;

        let params = TaskPaginationParams {
            page: Some(1),
            per_page: Some(10),
            search: Some("test".to_string()),
            completed: None,
            assignee: None,
        };

        let result = read_all(&db, &user_id, &params).await;
        assert!(result.is_ok());

        let tasks = result.unwrap();
        assert_eq!(tasks.len(), 2);
        assert!(tasks[0].title.contains("test"));
    }

    #[sqlx::test]
    async fn test_read_all_with_assignee_filter(db: PgPool) {
        let creator_id = create_test_user(&db, "creator", "creator@test.com").await;
        let assignee1_id = create_test_user(&db, "assignee1", "assignee1@test.com").await;
        let assignee2_id = create_test_user(&db, "assignee2", "assignee2@test.com").await;

        create_test_task(&db, &creator_id, &assignee1_id).await;
        create_test_task(&db, &creator_id, &assignee2_id).await;

        let params = TaskPaginationParams {
            page: Some(1),
            per_page: Some(10),
            search: None,
            completed: None,
            assignee: Some(assignee1_id.clone()),
        };

        let result = read_all(&db, &creator_id, &params).await;
        assert!(result.is_ok());

        let tasks = result.unwrap();
        assert_eq!(tasks.len(), 1);
    }

    #[sqlx::test]
    async fn test_update_task(db: PgPool) {
        let user_id = create_test_user(&db, "user", "user@test.com").await;

        let task_id = create_test_task(&db, &user_id, &user_id).await;

        let update_task = TaskUpdate {
            title: Some("Updated Task".to_string()),
            markdown: Some("# Updated\nThis task has been updated.".to_string()),
            completed: Some(true),
            due_date: Some(Utc::now()),
            assignee: None,
        };

        let result = update(&db, &task_id, &user_id, &update_task).await;
        assert!(result.is_ok());

        // Verify the update
        let updated_task = read_by_id(&db, &task_id, &user_id).await.unwrap();
        assert_eq!(updated_task.title, "Updated Task");
        assert_eq!(updated_task.completed, true);
    }

    #[sqlx::test]
    async fn test_update_task_unauthorized(db: PgPool) {
        let creator_id = create_test_user(&db, "creator", "creator@test.com").await;
        let assignee_id = create_test_user(&db, "assignee", "assignee@test.com").await;
        let other_user_id = create_test_user(&db, "other", "other@test.com").await;

        let task_id = create_test_task(&db, &creator_id, &assignee_id).await;

        let update_task = TaskUpdate {
            title: Some("Hacked Title".to_string()),
            markdown: None,
            completed: Some(true),
            due_date: None,
            assignee: None,
        };

        let result = update(&db, &task_id, &other_user_id, &update_task).await;
        assert!(result.is_ok()); // Query succeeds but affects 0 rows

        // Verify no changes were made
        let task = read_by_id(&db, &task_id, &creator_id).await.unwrap();
        assert_eq!(task.title, "test task");
    }

    #[sqlx::test]
    async fn test_delete_task_without_files(db: PgPool) {
        let user_id = create_test_user(&db, "user", "user@test.com").await;

        let task_id = create_test_task(&db, &user_id, &user_id).await;

        let delete_result = delete(&db, &task_id, &user_id, vec![]).await;
        assert!(delete_result.is_ok());

        // Verify task is deleted
        let find_result = read_by_id(&db, &task_id, &user_id).await;
        assert!(find_result.is_err());
    }

    #[sqlx::test]
    async fn test_delete_task_with_files(db: PgPool) {
        let user_id = create_test_user(&db, "user", "user@test.com").await;

        let task_id = create_test_task(&db, &user_id, &user_id).await;

        // Create test files
        let file1_id = create_test_file(&db, &user_id).await;
        let file2_id = create_test_file(&db, &user_id).await;

        // Associate files with task
        add_files(&db, &task_id, vec![file1_id.clone(), file2_id.clone()])
            .await
            .unwrap();

        let delete_result = delete(
            &db,
            &task_id,
            &user_id,
            vec![file1_id.clone(), file2_id.clone()],
        )
        .await;
        assert!(delete_result.is_ok());

        // Verify task is deleted
        let find_result = read_by_id(&db, &task_id, &user_id).await;
        assert!(find_result.is_err());

        // Verify files are deleted
        let file_count = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM files WHERE id = ANY($1)",
            &[file1_id, file2_id]
        )
        .fetch_one(&db)
        .await
        .unwrap();
        assert_eq!(file_count.unwrap_or(0), 0);
    }

    #[sqlx::test]
    async fn test_delete_system(db: PgPool) {
        let user_id = create_test_user(&db, "user", "user@test.com").await;

        let task_id = create_test_task(&db, &user_id, &user_id).await;
        let file_id = create_test_file(&db, &user_id).await;

        add_files(&db, &task_id, vec![file_id.clone()])
            .await
            .unwrap();

        let delete_result = delete_system(&db, &task_id, vec![file_id.clone()]).await;
        assert!(delete_result.is_ok());

        // Verify task is deleted (system delete bypasses user authorization)
        let find_result = read_by_id(&db, &task_id, &user_id).await;
        assert!(find_result.is_err());
    }

    #[sqlx::test]
    async fn test_add_files(db: PgPool) {
        let user_id = create_test_user(&db, "user", "user@test.com").await;

        let task_id = create_test_task(&db, &user_id, &user_id).await;

        // Create test files
        let file1_id = create_test_file(&db, &user_id).await;
        let file2_id = create_test_file(&db, &user_id).await;

        let result = add_files(&db, &task_id, vec![file1_id.clone(), file2_id.clone()]).await;
        assert!(result.is_ok());

        // Verify files are associated with task
        let file_count = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM task_files WHERE task_id = $1",
            task_id
        )
        .fetch_one(&db)
        .await
        .unwrap();
        assert_eq!(file_count.unwrap_or(0), 2);

        // Test duplicate insertion (should be ignored due to ON CONFLICT DO NOTHING)
        let result2 = add_files(&db, &task_id, vec![file1_id]).await;
        assert!(result2.is_ok());

        // Count should still be 2
        let file_count2 = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM task_files WHERE task_id = $1",
            task_id
        )
        .fetch_one(&db)
        .await
        .unwrap();
        assert_eq!(file_count2.unwrap_or(0), 2);
    }
}
