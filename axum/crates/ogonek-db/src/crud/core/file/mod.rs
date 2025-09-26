mod create;
mod delete;
mod multipart;
mod read;
mod update;

pub use create::*;
pub use delete::*;
pub use multipart::*;
pub use read::*;
pub use update::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        db::error::DbError,
        tests::create_test_user,
        types::{FileListParams, FileUpdate},
    };
    use sqlx::PgPool;

    async fn create_test_file(
        db: &PgPool,
        name: &str,
        owner_id: &str,
        parent_id: Option<&str>,
        is_folder: bool,
    ) -> String {
        let file_id = nanoid::nanoid!();
        let path = if let Some(parent_id) = parent_id {
            format!("/{}/{}", parent_id, name)
        } else {
            format!("/{}", name)
        };

        sqlx::query!(
            r#"
            INSERT INTO files (id, name, s3_key, path, mime_type, size, is_folder, parent_id, owner_id, visibility)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, 'private')
            "#,
            file_id,
            name,
            format!("s3-key-{}", file_id),
            path,
            if is_folder { None } else { Some("text/plain") },
            if is_folder { 0i64 } else { 1024i64 },
            is_folder,
            parent_id,
            owner_id
        )
        .execute(db)
        .await
        .unwrap();
        file_id
    }

    async fn create_test_task(db: &PgPool, title: &str, user_id: &str) -> String {
        let task_id = nanoid::nanoid!();
        sqlx::query!(
            r#"
            INSERT INTO tasks (id, title, markdown, created_by, assignee)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            task_id,
            title,
            "Test task description",
            user_id,
            user_id
        )
        .execute(db)
        .await
        .unwrap();
        task_id
    }

    async fn link_file_to_task(db: &PgPool, task_id: &str, file_id: &str) {
        sqlx::query!(
            r#"
            INSERT INTO task_files (task_id, file_id)
            VALUES ($1, $2)
            "#,
            task_id,
            file_id
        )
        .execute(db)
        .await
        .unwrap();
    }

    #[sqlx::test]
    async fn test_find_by_id_returns_file_for_owner(db: PgPool) {
        // Setup
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let file_id = create_test_file(&db, "test.txt", &user_id, None, false).await;

        // Test
        let result = find_by_id(&db, &file_id, &user_id).await;

        // Verify
        assert!(result.is_ok());
        let file = result.unwrap();
        assert_eq!(file.id, file_id);
        assert_eq!(file.name, "test.txt");
        assert_eq!(file.owner_id, user_id);
        assert!(!file.is_folder);
    }

    #[sqlx::test]
    async fn test_find_by_id_returns_not_found_for_non_owner(db: PgPool) {
        // Setup
        let user1_id = create_test_user(&db, "user1", "user1@example.com").await;
        let user2_id = create_test_user(&db, "user2", "user2@example.com").await;
        let file_id = create_test_file(&db, "test.txt", &user1_id, None, false).await;

        // Test - user2 trying to access user1's file
        let result = find_by_id(&db, &file_id, &user2_id).await;

        // Verify
        assert!(matches!(result, Err(DbError::NotFound(_))));
    }

    #[sqlx::test]
    async fn test_find_by_id_returns_not_found_for_nonexistent_file(db: PgPool) {
        // Setup
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let fake_file_id = nanoid::nanoid!();

        // Test
        let result = find_by_id(&db, &fake_file_id, &user_id).await;

        // Verify
        assert!(matches!(result, Err(DbError::NotFound(_))));
    }

    #[sqlx::test]
    async fn test_update_file_name_only(db: PgPool) {
        // Setup
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let file_id = create_test_file(&db, "old_name.txt", &user_id, None, false).await;

        let update_file = FileUpdate {
            name: Some("new_name.txt".to_string()),
            path: None,
            parent_id: None,
        };

        // Test
        let result = update(&db, &file_id, &user_id, update_file).await;

        // Verify
        assert!(result.is_ok());

        let file = find_by_id(&db, &file_id, &user_id).await.unwrap();
        assert_eq!(file.name, "new_name.txt");
    }

    #[sqlx::test]
    async fn test_update_file_path_only(db: PgPool) {
        // Setup
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let file_id = create_test_file(&db, "test.txt", &user_id, None, false).await;

        let update_file = FileUpdate {
            name: None,
            path: Some("/new/path/test.txt".to_string()),
            parent_id: None,
        };

        // Test
        let result = update(&db, &file_id, &user_id, update_file).await;

        // Verify
        assert!(result.is_ok());

        let file = find_by_id(&db, &file_id, &user_id).await.unwrap();
        assert_eq!(file.path, "/new/path/test.txt");
    }

    #[sqlx::test]
    async fn test_update_file_parent_id(db: PgPool) {
        // Setup
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let folder_id = create_test_file(&db, "folder", &user_id, None, true).await;
        let file_id = create_test_file(&db, "test.txt", &user_id, None, false).await;

        let update_file = FileUpdate {
            name: None,
            path: None,
            parent_id: Some(folder_id.clone()),
        };

        // Test
        let result = update(&db, &file_id, &user_id, update_file).await;

        // Verify
        assert!(result.is_ok());

        let file = find_by_id(&db, &file_id, &user_id).await.unwrap();
        assert_eq!(file.parent_id, Some(folder_id));
    }

    #[sqlx::test]
    async fn test_update_all_fields(db: PgPool) {
        // Setup
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let folder_id = create_test_file(&db, "folder", &user_id, None, true).await;
        let file_id = create_test_file(&db, "old.txt", &user_id, None, false).await;

        let update_file = FileUpdate {
            name: Some("new.txt".to_string()),
            path: Some("/folder/new.txt".to_string()),
            parent_id: Some(folder_id.clone()),
        };

        // Test
        let result = update(&db, &file_id, &user_id, update_file).await;

        // Verify
        assert!(result.is_ok());

        let file = find_by_id(&db, &file_id, &user_id).await.unwrap();
        assert_eq!(file.name, "new.txt");
        assert_eq!(file.path, "/folder/new.txt");
        assert_eq!(file.parent_id, Some(folder_id));
    }

    #[sqlx::test]
    async fn test_update_nonexistent_file(db: PgPool) {
        // Setup
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let fake_file_id = nanoid::nanoid!();

        let update_file = FileUpdate {
            name: Some("new_name.txt".to_string()),
            path: None,
            parent_id: None,
        };

        // Test
        let result = update(&db, &fake_file_id, &user_id, update_file).await;

        // Verify - should succeed but not update anything
        assert!(result.is_ok());
    }

    #[sqlx::test]
    async fn test_delete_returns_s3_key(db: PgPool) {
        // Setup
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let file_id = create_test_file(&db, "test.txt", &user_id, None, false).await;

        // Test
        let result = delete(&db, &file_id, &user_id).await;

        // Verify
        assert!(result.is_ok());
        let s3_record = result.unwrap();
        assert_eq!(s3_record.s3_key, Some(format!("s3-key-{}", file_id)));

        // Verify file is actually deleted
        let find_result = find_by_id(&db, &file_id, &user_id).await;
        assert!(matches!(find_result, Err(DbError::NotFound(_))));
    }

    #[sqlx::test]
    async fn test_delete_non_owner_file(db: PgPool) {
        // Setup
        let user1_id = create_test_user(&db, "user1", "user1@example.com").await;
        let user2_id = create_test_user(&db, "user2", "user2@example.com").await;
        let file_id = create_test_file(&db, "test.txt", &user1_id, None, false).await;

        // Test - user2 trying to delete user1's file
        let result = delete(&db, &file_id, &user2_id).await;

        // Verify - should return error (no rows affected)
        assert!(result.is_err());

        // File should still exist
        let find_result = find_by_id(&db, &file_id, &user1_id).await;
        assert!(find_result.is_ok());
    }

    #[sqlx::test]
    async fn test_delete_cascades_to_children(db: PgPool) {
        // Setup
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let folder_id = create_test_file(&db, "folder", &user_id, None, true).await;
        let child_file_id =
            create_test_file(&db, "child.txt", &user_id, Some(&folder_id), false).await;

        // Test - delete parent folder
        let result = delete(&db, &folder_id, &user_id).await;

        // Verify
        assert!(result.is_ok());

        // Both parent and child should be deleted
        assert!(matches!(
            find_by_id(&db, &folder_id, &user_id).await,
            Err(DbError::NotFound(_))
        ));
        assert!(matches!(
            find_by_id(&db, &child_file_id, &user_id).await,
            Err(DbError::NotFound(_))
        ));
    }

    #[sqlx::test]
    async fn test_find_all_root_files(db: PgPool) {
        // Setup
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let folder_id = create_test_file(&db, "folder", &user_id, None, true).await;
        let file_id = create_test_file(&db, "file.txt", &user_id, None, false).await;
        let _child_file =
            create_test_file(&db, "child.txt", &user_id, Some(&folder_id), false).await;

        let params = FileListParams { parent_id: None };

        // Test
        let result = find_all(&db, params, &user_id).await;

        // Verify
        assert!(result.is_ok());
        let files = result.unwrap();

        // Should only return root level files (folder and file.txt, not child.txt)
        assert_eq!(files.len(), 2);

        // Should be ordered by folders first, then name
        assert_eq!(files[0].id, folder_id);
        assert!(files[0].is_folder);
        assert_eq!(files[1].id, file_id);
        assert!(!files[1].is_folder);
    }

    #[sqlx::test]
    async fn test_find_all_folder_contents(db: PgPool) {
        // Setup
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let folder_id = create_test_file(&db, "folder", &user_id, None, true).await;
        let _root_file = create_test_file(&db, "root.txt", &user_id, None, false).await;
        let child_file1 =
            create_test_file(&db, "child1.txt", &user_id, Some(&folder_id), false).await;
        let child_file2 =
            create_test_file(&db, "child2.txt", &user_id, Some(&folder_id), false).await;

        let params = FileListParams {
            parent_id: Some(folder_id.clone()),
        };

        // Test
        let result = find_all(&db, params, &user_id).await;

        // Verify
        assert!(result.is_ok());
        let files = result.unwrap();

        // Should only return folder contents
        assert_eq!(files.len(), 2);
        let file_ids: Vec<&String> = files.iter().map(|f| &f.id).collect();
        assert!(file_ids.contains(&&child_file1));
        assert!(file_ids.contains(&&child_file2));
    }

    #[sqlx::test]
    async fn test_find_all_nonexistent_folder(db: PgPool) {
        // Setup
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let fake_folder_id = nanoid::nanoid!();

        let params = FileListParams {
            parent_id: Some(fake_folder_id),
        };

        // Test
        let result = find_all(&db, params, &user_id).await;

        // Verify - should return error because folder doesn't exist
        assert!(matches!(result, Err(DbError::NotFound(_))));
    }

    #[sqlx::test]
    async fn test_find_all_empty_folder(db: PgPool) {
        // Setup
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let folder_id = create_test_file(&db, "empty_folder", &user_id, None, true).await;

        let params = FileListParams {
            parent_id: Some(folder_id),
        };

        // Test
        let result = find_all(&db, params, &user_id).await;

        // Verify
        assert!(result.is_ok());
        let files = result.unwrap();
        assert_eq!(files.len(), 0);
    }

    #[sqlx::test]
    async fn test_check_file_exists_success(db: PgPool) {
        // Setup
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let file_id = create_test_file(&db, "test.txt", &user_id, None, false).await;

        // Test
        let result = check_file_exists(&db, &file_id, &user_id).await;

        // Verify
        assert!(result.is_ok());
    }

    #[sqlx::test]
    async fn test_check_file_exists_not_found(db: PgPool) {
        // Setup
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let fake_file_id = nanoid::nanoid!();

        // Test
        let result = check_file_exists(&db, &fake_file_id, &user_id).await;

        // Verify
        assert!(matches!(result, Err(DbError::NotFound(_))));
    }

    #[sqlx::test]
    async fn test_check_file_exists_wrong_owner(db: PgPool) {
        // Setup
        let user1_id = create_test_user(&db, "user1", "user1@example.com").await;
        let user2_id = create_test_user(&db, "user2", "user2@example.com").await;
        let file_id = create_test_file(&db, "test.txt", &user1_id, None, false).await;

        // Test
        let result = check_file_exists(&db, &file_id, &user2_id).await;

        // Verify
        assert!(matches!(result, Err(DbError::NotFound(_))));
    }

    #[sqlx::test]
    async fn test_fetch_files_task_returns_linked_files(db: PgPool) {
        // Setup
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let task_id = create_test_task(&db, "Test Task", &user_id).await;

        let file1_id = create_test_file(&db, "file1.txt", &user_id, None, false).await;
        let file2_id = create_test_file(&db, "file2.txt", &user_id, None, false).await;
        let _unlinked_file = create_test_file(&db, "unlinked.txt", &user_id, None, false).await;

        link_file_to_task(&db, &task_id, &file1_id).await;
        link_file_to_task(&db, &task_id, &file2_id).await;

        // Test
        let result = fetch_files_task(&db, &task_id).await;

        // Verify
        assert!(result.is_ok());
        let files = result.unwrap();
        assert_eq!(files.len(), 2);

        let file_ids: Vec<&String> = files.iter().map(|f| &f.id).collect();
        assert!(file_ids.contains(&&file1_id));
        assert!(file_ids.contains(&&file2_id));

        // Verify FileSmall fields
        let file = &files[0];
        assert_eq!(file.owner_id, user_id);
        assert!(file.name.contains("file"));
        assert_eq!(file.mime_type, Some("text/plain".to_string()));
        assert_eq!(file.size, 1024);
    }

    #[sqlx::test]
    async fn test_fetch_files_task_empty_task(db: PgPool) {
        // Setup
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let task_id = create_test_task(&db, "Empty Task", &user_id).await;

        // Test
        let result = fetch_files_task(&db, &task_id).await;

        // Verify
        assert!(result.is_ok());
        let files = result.unwrap();
        assert_eq!(files.len(), 0);
    }

    #[sqlx::test]
    async fn test_fetch_files_task_nonexistent_task(db: PgPool) {
        // Setup
        let fake_task_id = nanoid::nanoid!();

        // Test
        let result = fetch_files_task(&db, &fake_task_id).await;

        // Verify
        assert!(result.is_ok());
        let files = result.unwrap();
        assert_eq!(files.len(), 0);
    }

    #[sqlx::test]
    async fn test_file_ordering_folders_first(db: PgPool) {
        // Setup
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;

        // Create files in alphabetical order but mixed types
        let _file_a = create_test_file(&db, "a_file.txt", &user_id, None, false).await;
        let folder_b = create_test_file(&db, "b_folder", &user_id, None, true).await;
        let _file_c = create_test_file(&db, "c_file.txt", &user_id, None, false).await;
        let folder_d = create_test_file(&db, "d_folder", &user_id, None, true).await;

        let params = FileListParams { parent_id: None };

        // Test
        let result = find_all(&db, params, &user_id).await;

        // Verify
        assert!(result.is_ok());
        let files = result.unwrap();
        assert_eq!(files.len(), 4);

        // First two should be folders (in alphabetical order)
        assert!(files[0].is_folder);
        assert_eq!(files[0].id, folder_b);
        assert!(files[1].is_folder);
        assert_eq!(files[1].id, folder_d);

        // Last two should be files (in alphabetical order)
        assert!(!files[2].is_folder);
        assert_eq!(files[2].name, "a_file.txt");
        assert!(!files[3].is_folder);
        assert_eq!(files[3].name, "c_file.txt");
    }
}
