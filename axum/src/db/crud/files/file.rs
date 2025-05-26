use crate::db::error::DbError;
use crate::models::files::FileSmall;
use crate::models::{File, FileListParams, FileUpdate, S3KeyRecord};
use sqlx::PgPool;

pub async fn find_by_id(db: &PgPool, id: &str, user_id: &str) -> Result<File, DbError> {
    let file = sqlx::query_as!(
        File,
        r#"
            SELECT * FROM files
            WHERE id = $1 AND (owner_id = $2)
            "#,
        id,
        user_id
    )
    .fetch_optional(db)
    .await?
    .ok_or_else(|| DbError::NotFound("File not found".into()))?;

    Ok(file)
}

pub async fn update(
    db: &PgPool,
    file_id: &str,
    user_id: &str,
    update: FileUpdate,
) -> Result<(), DbError> {
    let parent_id = if let Some(parent_id) = update.parent_id {
        check_file_exists(db, file_id, user_id).await?;
        Some(parent_id)
    } else {
        None
    };

    sqlx::query!(
        r#"
        UPDATE files
        SET
            name = COALESCE($3, name),
            path = COALESCE($4, path),
            parent_id = COALESCE($5, parent_id)
        WHERE id=$1 AND owner_id = $2
        "#,
        file_id,
        user_id,
        update.name,
        update.path,
        parent_id
    )
    .execute(db)
    .await?;

    Ok(())
}

pub async fn delete(db: &PgPool, file_id: &str, user_id: &str) -> Result<S3KeyRecord, DbError> {
    let file = sqlx::query_as!(
        S3KeyRecord,
        r#"
        DELETE FROM files
        WHERE id = $1 AND owner_id = $2
        RETURNING s3_key
        "#,
        file_id,
        user_id
    )
    .fetch_one(db)
    .await?;

    Ok(file)
}

pub async fn find_all(
    db: &PgPool,
    params: FileListParams,
    user_id: &str,
) -> Result<Vec<File>, DbError> {
    let files = if let Some(folder_id) = params.parent_id {
        check_file_exists(db, &folder_id, user_id).await?;

        sqlx::query_as!(
            File,
            r#"
            SELECT * FROM files
            WHERE parent_id = $1 AND owner_id = $2
            ORDER BY is_folder DESC, name ASC
            "#,
            folder_id,
            user_id
        )
        .fetch_all(db)
        .await?
    } else {
        // Root folder contents
        sqlx::query_as!(
            File,
            r#"
            SELECT * FROM files
            WHERE parent_id IS NULL AND owner_id = $1
            ORDER BY is_folder DESC, name ASC
            "#,
            user_id
        )
        .fetch_all(db)
        .await?
    };
    Ok(files)
}
pub async fn check_file_exists(db: &PgPool, file_id: &str, user_id: &str) -> Result<(), DbError> {
    let file_exists = sqlx::query!(
        r#"
        SELECT 1 as "exists!: bool" FROM files
        WHERE id = $1 AND owner_id = $2
        "#,
        file_id,
        user_id
    )
    .fetch_optional(db)
    .await?
    .is_some();

    if !file_exists {
        return Err(DbError::NotFound("File not found".into()));
    }

    Ok(())
}

pub async fn fetch_files_task(db: &PgPool, id: &str) -> Result<Vec<FileSmall>, DbError> {
    let files = sqlx::query_as!(
        FileSmall,
        r#"
        SELECT
            f.id,
            f.name,
            f.mime_type,
            f.s3_key,
            f.size,
            f.owner_id
        FROM files f
        JOIN task_files tf ON f.id = tf.file_id
        WHERE tf.task_id = $1
        "#,
        id
    )
    .fetch_all(db)
    .await?;

    Ok(files)
}
