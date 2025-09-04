use crate::{
    db::error::DbError,
    types::{Photo, UpsertPhoto},
};
use serde_json::json;

/// Returns the lesson's photo
pub async fn find_by_id(
    db: impl sqlx::Executor<'_, Database = sqlx::Postgres>,
    photo_id: &str,
) -> Result<Option<Photo>, DbError> {
    let photo = sqlx::query_as!(
        Photo,
        r#"

        SELECT * FROM photos WHERE id = $1
        
        "#,
        photo_id
    )
    .fetch_optional(db)
    .await?;

    Ok(photo)
}
use sqlx::PgPool;

/// Upserts photo
pub async fn upsert(
    db: &PgPool,
    lesson_id: &str,
    user_id: &str,
    update: &UpsertPhoto
) -> Result<(), DbError> {
    let photo_id = nanoid::nanoid!();

    let mut tx = db.begin().await?;
    let photo_id = sqlx::query_scalar!(
        r#"
        INSERT INTO photos (id, unsplash_id, urls, alt_description, photographer_name, photographer_username)
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (unsplash_id) 
        DO UPDATE SET
            urls = EXCLUDED.urls,
            alt_description = EXCLUDED.alt_description,
            photographer_name = EXCLUDED.photographer_name,
            photographer_username = EXCLUDED.photographer_username
        RETURNING id
        "#,
        photo_id,
        update.unsplash_id,
        json!(update.urls), 
        update.alt_description,
        update.user.name,
        update.user.username
    )
    .fetch_one(&mut *tx)
    .await?;

    sqlx::query!(
        r#"
        UPDATE lessons
        SET photo_id = COALESCE($2, photo_id)
        WHERE id = $1 AND created_by = $3
        "#,
        lesson_id,
        photo_id,
        user_id

    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(())
}

/// Deletes photo by lesson id
pub async fn delete(
    db: &PgPool,
    lesson_id: &str,
    user_id: &str,
) -> Result<(), DbError> {
    sqlx::query!(
        r#"
        DELETE FROM photos
        WHERE id = (
        SELECT photo_id
        FROM lessons
        WHERE id = $1 AND created_by = $2
    )
        "#,
        lesson_id,
        user_id

    )
    .execute(db)
    .await?;

    Ok(())
}
