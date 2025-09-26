use crate::DbError;
use ogonek_types::S3KeyRecord;
use sqlx::PgPool;

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
