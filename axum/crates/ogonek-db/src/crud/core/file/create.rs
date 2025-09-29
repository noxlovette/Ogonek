use crate::DbError;
use sqlx::PgPool;

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
