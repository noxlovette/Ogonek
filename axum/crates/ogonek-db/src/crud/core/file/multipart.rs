use crate::DbError;
use ogonek_types::{FileCreateParams, FileLinkOptions};
use sqlx::{PgPool, Postgres, Transaction};
/// Creates a new file record in the database with pending upload status
pub async fn create_pending_file(
    tx: &mut Transaction<'_, Postgres>,
    params: &FileCreateParams,
) -> Result<(), DbError> {
    let path = format!("/{}", params.file_name);

    sqlx::query!(
        r#"
        INSERT INTO files (
            id, name, s3_key, path, mime_type, size, is_folder, 
            parent_id, owner_id, visibility, upload_status
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        "#,
        params.file_id,
        params.file_name,
        params.s3_key,
        path,
        Some(&params.content_type),
        params.file_size,
        false,
        params.parent_id,
        params.owner_id,
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
    params: FileCreateParams,
    options: FileLinkOptions,
) -> Result<(), DbError> {
    let mut tx = db.begin().await?;

    create_pending_file(&mut tx, &params).await?;

    if let Some(task_id) = options.task_id {
        link_file_to_task(&mut tx, &task_id, &params.file_id).await?;
    }

    tx.commit().await?;
    Ok(())
}
