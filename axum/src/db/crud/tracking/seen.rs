use crate::db::error::DbError;
use crate::types::ModelType;
use sqlx::PgPool;

/// Lets mark an object as seen
pub async fn mark_as_seen(
    db: impl sqlx::Executor<'_, Database = sqlx::Postgres>,
    user_id: &str,
    model_id: &str,
    model_type: ModelType,
) -> Result<(), DbError> {
    sqlx::query!(
        r#"
        INSERT INTO seen_status (user_id, model_type, model_id, seen_at)
        VALUES ($1, $2, $3, now())
        ON CONFLICT (user_id, model_type, model_id) DO UPDATE
        SET seen_at = EXCLUDED.seen_at;
        "#,
        user_id,
        model_type.as_str(),
        model_id
    )
    .execute(db)
    .await?;
    Ok(())
}

pub async fn delete_seen(
    db: &PgPool,
    user_id: &str,
    model_id: &str,
    model_type: ModelType,
) -> Result<(), DbError> {
    sqlx::query!(
        "DELETE FROM seen_status WHERE user_id = $1 AND model_type = $2 AND model_id = $3",
        user_id,
        model_type.as_str(),
        model_id
    )
    .execute(db)
    .await?;

    Ok(())
}

/// Will return how many of the selected model type and user have not been seen yet
pub async fn get_seen_badge(
    db: &PgPool,
    user_id: &str,
    model_type: ModelType,
) -> Result<i64, DbError> {
    let count = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*) FROM seen_status
        WHERE user_id = $1
          AND model_type = $2
          AND seen_at IS NULL
        "#,
        user_id,
        model_type.as_str(),
    )
    .fetch_one(db)
    .await?;

    Ok(count.unwrap_or(0))
}

/// Insert a new entry into seen_status with seen_at set to NULL
/// This marks an item as "not seen" when it's first created
pub async fn insert_as_unseen(
    db: &PgPool,
    user_id: &str,
    model_id: &str,
    model_type: ModelType,
) -> Result<(), DbError> {
    sqlx::query!(
        r#"
        INSERT INTO seen_status (user_id, model_type, model_id, seen_at)
        VALUES ($1, $2, $3, NULL)
        ON CONFLICT (user_id, model_type, model_id) DO NOTHING;
        "#,
        user_id,
        model_type.as_str(),
        model_id
    )
    .execute(db)
    .await?;
    Ok(())
}
