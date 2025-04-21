use crate::db::error::DbError;
use crate::models::files::FileSmall;
use sqlx::PgPool;

pub async fn check_file_exists(db: &PgPool, file_id: &str, user_id: &str) -> Result<(), DbError> {
    let file_exists = sqlx::query!(
        r#"
        SELECT 1 as "exists!: bool" FROM files
        WHERE id = $1 AND owner_id = $2 AND upload_status = 'pending'
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
