use crate::db::error::DbError;
use sqlx::{PgPool, Postgres, Transaction};

/// Creates a new file record in the database with pending upload status
pub async fn create_pending_file(
    tx: &mut Transaction<'_, Postgres>,
    file_id: &str,
    file_name: &str,
    s3_key: &str,
    content_type: &str,
    file_size: i64,
    parent_id: Option<&str>,
    owner_id: &str,
) -> Result<(), DbError> {
    let path = format!("/{}", file_name);

    sqlx::query!(
        r#"
        INSERT INTO files (
            id, name, s3_key, path, mime_type, size, is_folder, 
            parent_id, owner_id, visibility, upload_status
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        "#,
        file_id,
        file_name,
        s3_key,
        path,
        Some(content_type),
        file_size,
        false,
        parent_id,
        owner_id,
        "private",
        "pending"
    )
    .execute(&mut **tx)
    .await?;

    Ok(())
}

/// Links a file to a task (upsert behavior with ON CONFLICT DO NOTHING)
pub async fn link_file_to_task(
    tx: &mut Transaction<'_, Postgres>,
    task_id: &str,
    file_id: &str,
) -> Result<(), DbError> {
    sqlx::query!(
        r#"
        INSERT INTO task_files (task_id, file_id)
        VALUES ($1, $2)
        ON CONFLICT DO NOTHING
        "#,
        task_id,
        file_id
    )
    .execute(&mut **tx)
    .await?;

    Ok(())
}

/// Marks a file upload as complete
pub async fn mark_upload_complete(
    db: &PgPool,
    file_id: &str,
    owner_id: &str,
) -> Result<(), DbError> {
    let rows_affected = sqlx::query!(
        r#"
        UPDATE files
        SET upload_status = 'complete'
        WHERE id = $1 AND owner_id = $2
        "#,
        file_id,
        owner_id
    )
    .execute(db)
    .await?
    .rows_affected();

    // Ensure the file actually existed and was updated
    if rows_affected == 0 {
        return Err(DbError::NotFound(
            "File not found or unauthorized".to_string(),
        ));
    }

    Ok(())
}

/// Creates a complete multipart upload transaction
/// This is a higher-level function that combines file creation and optional task linking
pub async fn create_multipart_file(
    db: &PgPool,
    file_id: &str,
    file_name: &str,
    s3_key: &str,
    content_type: &str,
    file_size: i64,
    parent_id: Option<&str>,
    owner_id: &str,
    task_id: Option<&str>,
) -> Result<(), DbError> {
    let mut tx = db.begin().await?;

    create_pending_file(
        &mut tx,
        file_id,
        file_name,
        s3_key,
        content_type,
        file_size,
        parent_id,
        owner_id,
    )
    .await?;

    if let Some(task_id) = task_id {
        link_file_to_task(&mut tx, task_id, file_id).await?;
    }

    tx.commit().await?;
    Ok(())
}
