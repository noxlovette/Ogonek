use crate::db::error::DbError;
use crate::types::{TaskCreate, TaskFull, TaskPaginationParams, TaskSmall, TaskUpdate};
use sqlx::PgPool;

/// Mini-tasks
pub async fn find_all(
    db: &PgPool,
    user_id: &str,
    params: &TaskPaginationParams,
) -> Result<Vec<TaskSmall>, DbError> {
    let mut query_builder = sqlx::QueryBuilder::new(
        r#"SELECT
                t.id,
                t.title,
                t.priority,
                t.completed,
                t.due_date,
                u.name as assignee_name,
                COALESCE(s.seen_at IS NOT NULL, TRUE) as seen
            FROM tasks t
            LEFT JOIN "user" u ON t.assignee = u.id
            LEFT JOIN seen_status s ON s.model_id = t.id AND s.user_id = "#,
    );
    query_builder.push_bind(user_id);
    query_builder.push(
        r#"
            WHERE (t.assignee = "#,
    );
    query_builder.push_bind(user_id);
    query_builder.push(" OR t.created_by = ");
    query_builder.push_bind(user_id);
    query_builder.push(")");

    // Add search filter if provided
    if let Some(search) = &params.search {
        query_builder.push(" AND (t.title ILIKE ");
        query_builder.push_bind(format!("%{search}%"));
        query_builder.push(" OR t.markdown ILIKE ");
        query_builder.push_bind(format!("%{search}%"));
        query_builder.push(")");
    }

    if let Some(completed) = &params.completed {
        query_builder.push(" AND t.completed = ");
        query_builder.push_bind(completed);
    }

    if let Some(assignee) = &params.assignee {
        query_builder.push(" AND t.assignee = ");
        query_builder.push_bind(assignee);
    }

    // Add ordering - tasks typically ordered by due date
    query_builder.push(" ORDER BY t.due_date ASC NULLS LAST");

    // Add limit and offset
    query_builder.push(" LIMIT ");
    query_builder.push_bind(params.limit());
    query_builder.push(" OFFSET ");
    query_builder.push_bind(params.offset());

    // Execute the query
    let tasks = query_builder
        .build_query_as::<TaskSmall>()
        .fetch_all(db)
        .await?;
    Ok(tasks)
}

/// Returns the full Task with all fields
pub async fn find_by_id(db: &PgPool, id: &str, user_id: &str) -> Result<TaskFull, DbError> {
    let task = sqlx::query_as!(
        TaskFull,
        r#"
            SELECT
                t.id,
                t.title,
                t.markdown,
                t.priority,
                t.completed,
                t.due_date,
                t.assignee,
                t.created_by,
                t.created_at,
                t.updated_at,
                u.name as assignee_name
            FROM tasks t
            LEFT JOIN "user" u ON t.assignee = u.id
            WHERE t.id = $1
            AND (t.assignee = $2 OR t.created_by = $2)
            "#,
        id,
        user_id,
    )
    .fetch_optional(db)
    .await?
    .ok_or_else(|| DbError::NotFound("Task not found".into()))?;

    Ok(task)
}

/// Creates a task, needs data to create FROM
pub async fn create(
    db: &PgPool,
    task: &TaskCreate,
    user_id: &str,
    assignee: &str,
) -> Result<String, DbError> {
    let id = sqlx::query_scalar!(
        "INSERT INTO tasks (id, title, markdown, due_date, assignee, created_by)
         VALUES ($1, $2, $3, $4, $5, $6)
         RETURNING id
         ",
        nanoid::nanoid!(),
        task.title,
        task.markdown,
        task.due_date,
        assignee,
        user_id,
    )
    .fetch_one(db)
    .await?;

    Ok(id)
}
/// Creates a task based on user preferences, faster – no JSON
pub async fn create_with_defaults(db: &PgPool, user_id: &str) -> Result<String, DbError> {
    let id = sqlx::query_scalar!(
        "INSERT INTO tasks (id, title, markdown, assignee, created_by)
         VALUES ($1, $2, $3, $4, $5 )
         RETURNING id
         ",
        nanoid::nanoid!(),
        "Default Title", // TODO: feed in preferred title
        "# Default markdown",
        user_id,
        user_id,
    )
    .fetch_one(db)
    .await?;

    Ok(id)
}

/// Deletes a task
pub async fn delete(
    db: &PgPool,
    id: &str,
    user_id: &str,
    file_ids: Vec<String>,
) -> Result<(), DbError> {
    let mut tx = db.begin().await?;

    if !file_ids.is_empty() {
        sqlx::query!(r#"DELETE FROM task_files WHERE task_id = $1"#, id)
            .execute(&mut *tx)
            .await?;

        sqlx::query!(r#"DELETE FROM files WHERE id = ANY($1)"#, &file_ids)
            .execute(&mut *tx)
            .await?;
    }

    sqlx::query!(
        r#"DELETE FROM tasks WHERE id = $1 AND (assignee = $2 OR created_by = $2)"#,
        id,
        user_id
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(())
}

pub async fn delete_system(db: &PgPool, id: &str, file_ids: Vec<String>) -> Result<(), DbError> {
    let mut tx = db.begin().await?;

    if !file_ids.is_empty() {
        sqlx::query!(r#"DELETE FROM task_files WHERE task_id = $1"#, id)
            .execute(&mut *tx)
            .await?;

        sqlx::query!(r#"DELETE FROM files WHERE id = ANY($1)"#, &file_ids)
            .execute(&mut *tx)
            .await?;
    }

    sqlx::query!(r#"DELETE FROM tasks WHERE id = $1"#, id)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    Ok(())
}

/// Finds the assignee for the task
pub async fn find_assignee(
    db: &PgPool,
    lesson_id: &str,
    user_id: &str,
) -> Result<Option<String>, DbError> {
    let assignee = sqlx::query_scalar!(
        r#"
        SELECT assignee
        FROM tasks
        WHERE id = $1
        AND (assignee = $2 OR created_by = $2)
        "#,
        lesson_id,
        user_id
    )
    .fetch_optional(db)
    .await?;

    Ok(assignee)
}

/// Finds the assignee for the task
pub async fn toggle(db: &PgPool, task_id: &str, user_id: &str) -> Result<bool, DbError> {
    let completed = sqlx::query_scalar!(
        r#"
        SELECT completed
        FROM tasks
        WHERE id = $1
        AND (assignee = $2 OR created_by = $2)
        "#,
        task_id,
        user_id
    )
    .fetch_one(db)
    .await?;

    sqlx::query!(
        r#"
       UPDATE tasks
       SET
        completed = $3
         WHERE id = $1 AND (assignee = $2 OR created_by = $2)
       "#,
        task_id,
        user_id,
        !completed
    )
    .execute(db)
    .await?;

    Ok(!completed)
}

/// Updates the task and inserts associated files
pub async fn update(
    db: &PgPool,
    id: &str,
    user_id: &str,
    update: &TaskUpdate,
) -> Result<(), DbError> {
    sqlx::query!(
        "UPDATE tasks
         SET
            title = COALESCE($3, title),
            markdown = COALESCE($4, markdown),
            priority = COALESCE($5, priority),
            completed = COALESCE($6, completed),
            due_date = COALESCE($7, due_date),
            assignee = COALESCE($8, assignee)
         WHERE id = $1 AND (assignee = $2 OR created_by = $2)",
        id,
        user_id,
        update.title,
        update.markdown,
        update.priority,
        update.completed,
        update.due_date,
        update.assignee,
    )
    .execute(db)
    .await?;

    Ok(())
}
pub async fn count(db: &PgPool, user_id: &str) -> Result<i64, DbError> {
    let count = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM tasks WHERE
            (created_by = $1 OR assignee = $1)
            ",
        user_id
    )
    .fetch_one(db)
    .await?;

    Ok(count.unwrap_or(0))
}

pub async fn find_recent(db: &PgPool, user_id: &str) -> Result<Vec<TaskSmall>, DbError> {
    let tasks = sqlx::query_as!(
        TaskSmall,
        r#"
        SELECT
                t.id,
                t.title,
                t.priority,
                t.completed,
                t.due_date,
                u.name as assignee_name,
               COALESCE(s.seen_at IS NOT NULL, TRUE) as seen
            FROM tasks t
            LEFT JOIN "user" u ON t.assignee = u.id
            LEFT JOIN seen_status s ON s.model_id = t.id AND s.user_id = $1
        WHERE (assignee = $1 OR created_by = $1)
        AND completed = false
        ORDER BY created_at DESC
        LIMIT 3
        "#,
        user_id
    )
    .fetch_all(db)
    .await?;

    Ok(tasks)
}

pub async fn add_files(db: &PgPool, task_id: &str, file_ids: Vec<String>) -> Result<(), DbError> {
    let mut tx = db.begin().await?;

    for file_id in file_ids {
        sqlx::query!(
            r#"
            INSERT INTO task_files (task_id, file_id)
            VALUES ($1, $2)
            ON CONFLICT DO NOTHING
            "#,
            task_id,
            file_id,
        )
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;

    Ok(())
}

pub async fn fetch_old_tasks(db: &PgPool) -> Result<Vec<String>, DbError> {
    let tasks = sqlx::query_scalar!(
        r#"
        SELECT id
        FROM tasks
        WHERE created_at < NOW() - INTERVAL '1 month'
        AND completed = true;
        "#
    )
    .fetch_all(db)
    .await?;

    Ok(tasks)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::create_test_user;
    use crate::types::{TaskCreate, TaskPaginationParams, TaskUpdate};
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
    async fn test_find_by_id_as_creator(db: PgPool) {
        let creator_id = create_test_user(&db, "creator", "creator@test.com").await;
        let assignee_id = create_test_user(&db, "assignee", "assignee@test.com").await;

        let task_id = create_test_task(&db, &creator_id, &assignee_id).await;

        let result = find_by_id(&db, &task_id, &creator_id).await;
        assert!(result.is_ok());

        let task = result.unwrap();
        assert_eq!(task.title, "test task");
        assert_eq!(task.created_by, creator_id);
        assert_eq!(task.assignee, assignee_id);
    }

    #[sqlx::test]
    async fn test_find_by_id_as_assignee(db: PgPool) {
        let creator_id = create_test_user(&db, "creator", "creator@test.com").await;
        let assignee_id = create_test_user(&db, "assignee", "assignee@test.com").await;

        let task_id = create_test_task(&db, &creator_id, &assignee_id).await;
        let result = find_by_id(&db, &task_id, &assignee_id).await;
        assert!(result.is_ok());

        let task = result.unwrap();
        assert_eq!(task.assignee, assignee_id);
    }

    #[sqlx::test]
    async fn test_find_by_id_unauthorized(db: PgPool) {
        let creator_id = create_test_user(&db, "creator", "creator@test.com").await;
        let assignee_id = create_test_user(&db, "assignee", "assignee@test.com").await;
        let other_user_id = create_test_user(&db, "other", "other@test.com").await;
        let task_id = create_test_task(&db, &creator_id, &assignee_id).await;
        let result = find_by_id(&db, &task_id, &other_user_id).await;
        assert!(result.is_err());
    }

    #[sqlx::test]
    async fn test_find_all_basic(db: PgPool) {
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

        let result = find_all(&db, &user_id, &params).await;
        assert!(result.is_ok());

        let tasks = result.unwrap();
        assert_eq!(tasks.len(), 3);
    }

    #[sqlx::test]
    async fn test_find_all_with_search(db: PgPool) {
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

        let result = find_all(&db, &user_id, &params).await;
        assert!(result.is_ok());

        let tasks = result.unwrap();
        assert_eq!(tasks.len(), 2);
        assert!(tasks[0].title.contains("test"));
    }

    #[sqlx::test]
    async fn test_find_all_with_assignee_filter(db: PgPool) {
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

        let result = find_all(&db, &creator_id, &params).await;
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
            priority: Some(1),
            completed: Some(true),
            due_date: Some(Utc::now()),
            assignee: None,
        };

        let result = update(&db, &task_id, &user_id, &update_task).await;
        assert!(result.is_ok());

        // Verify the update
        let updated_task = find_by_id(&db, &task_id, &user_id).await.unwrap();
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
            priority: Some(2),
            completed: Some(true),
            due_date: None,
            assignee: None,
        };

        let result = update(&db, &task_id, &other_user_id, &update_task).await;
        assert!(result.is_ok()); // Query succeeds but affects 0 rows

        // Verify no changes were made
        let task = find_by_id(&db, &task_id, &creator_id).await.unwrap();
        assert_eq!(task.title, "test task");
    }

    #[sqlx::test]
    async fn test_delete_task_without_files(db: PgPool) {
        let user_id = create_test_user(&db, "user", "user@test.com").await;

        let task_id = create_test_task(&db, &user_id, &user_id).await;

        let delete_result = delete(&db, &task_id, &user_id, vec![]).await;
        assert!(delete_result.is_ok());

        // Verify task is deleted
        let find_result = find_by_id(&db, &task_id, &user_id).await;
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
        let find_result = find_by_id(&db, &task_id, &user_id).await;
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
        let find_result = find_by_id(&db, &task_id, &user_id).await;
        assert!(find_result.is_err());
    }

    #[sqlx::test]
    async fn test_fetch_recent(db: PgPool) {
        let user_id = create_test_user(&db, "user", "user@test.com").await;

        // Create multiple tasks (some completed, some not)
        for i in 1..=5 {
            let task_id = create_test_task(&db, &user_id, &user_id).await;

            // Mark some as completed
            if i % 2 == 0 {
                let update_task = TaskUpdate {
                    title: None,
                    markdown: None,
                    priority: None,
                    completed: Some(true),
                    due_date: None,
                    assignee: None,
                };
                update(&db, &task_id, &user_id, &update_task).await.unwrap();
            }
        }

        let result = find_recent(&db, &user_id).await;
        assert!(result.is_ok());

        let tasks = result.unwrap();
        assert_eq!(tasks.len(), 3); // Should be limited to 3 incomplete tasks

        // All tasks should be incomplete
        for task in &tasks {
            assert_eq!(task.completed, false);
        }
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
