use sqlx::PgPool;

use crate::DbError;

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

pub async fn delete_many(pool: &PgPool, ids: Vec<String>, user_id: &str) -> Result<u64, DbError> {
    let result = sqlx::query!(
        r#"
        DELETE FROM lessons
        WHERE id = ANY($1) AND created_by = $2
        "#,
        &ids,
        user_id
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}
