use crate::DbError;
use sqlx::PgPool;
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

pub async fn delete_many(pool: &PgPool, ids: Vec<String>, user_id: &str) -> Result<u64, DbError> {
    let result = sqlx::query!(
        r#"
        DELETE FROM tasks 
        WHERE id = ANY($1) AND created_by = $2
        "#,
        &ids,
        user_id
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}
