use sqlx::PgPool;

use crate::{
    db::error::DbError,
    types::{Content, ContentPublic, ContentStatus, UpdateContent},
};

/// For public endpoints that renders content
pub async fn find_by_slug(db: &PgPool, slug: &str) -> Result<ContentPublic, DbError> {
    let content = sqlx::query_as!(
        ContentPublic,
        r#"
        SELECT title, markdown, meta_description
        FROM content
        WHERE slug = $1 AND status = 'published'
        "#,
        slug
    )
    .fetch_one(db)
    .await?;

    Ok(content)
}

/// For admin interfaces
pub async fn find_by_id(db: &PgPool, id: &str) -> Result<Content, DbError> {
    let content = sqlx::query_as!(
        Content,
        r#"
        SELECT
        id,
        slug,
        title,
        markdown,
        meta_description,
        version,
        status as "status: ContentStatus",
        published_at,
        updated_at,
        updated_by
        FROM content
        WHERE id = $1 
        "#,
        id
    )
    .fetch_one(db)
    .await?;

    Ok(content)
}

/// For admin interfaces
pub async fn update(
    db: &PgPool,
    content_id: &str,
    user_id: &str,
    update: &UpdateContent,
) -> Result<(), DbError> {
    sqlx::query!(
        r#"
      UPDATE content
      SET
        slug = COALESCE($3, slug),
        title = COALESCE($4, title),
        markdown = COALESCE($5, markdown),
        meta_description = COALESCE($6, meta_description),
        version = version + 1,
        updated_at = NOW(),
        updated_by = $2
         WHERE id = $1 
        "#,
        content_id,
        user_id,
        update.slug,
        update.title,
        update.markdown,
        update.meta_description,
    )
    .execute(db)
    .await?;

    Ok(())
}

/// For admin interfaces
pub async fn publish(db: &PgPool, content_id: &str, user_id: &str) -> Result<(), DbError> {
    sqlx::query!(
        r#"
      UPDATE content
      SET
         published_at = NOW(),
         status = 'published',
         updated_by = $2
      WHERE id = $1 
        "#,
        content_id,
        user_id,
    )
    .execute(db)
    .await?;

    Ok(())
}

/// For admin interfaces
pub async fn unpublish(db: &PgPool, content_id: &str, user_id: &str) -> Result<(), DbError> {
    sqlx::query!(
        r#"
      UPDATE content
      SET
         published_at = NULL,
         status = 'draft',
         updated_by = $2
      WHERE id = $1 
        "#,
        content_id,
        user_id,
    )
    .execute(db)
    .await?;

    Ok(())
}

/// For admin interfaces
pub async fn delete(db: &PgPool, content_id: &str) -> Result<(), DbError> {
    sqlx::query!(
        r#"
      DELETE FROM content
      WHERE id = $1 
        "#,
        content_id,
    )
    .execute(db)
    .await?;

    Ok(())
}

/// For admin interfaces
pub async fn find_all(db: &PgPool) -> Result<Vec<Content>, DbError> {
    let content = sqlx::query_as!(
        Content,
        r#"
        SELECT
        id,
        slug,
        title,
        markdown,
        meta_description,
        version,
        status as "status: ContentStatus",
        published_at,
        updated_at,
        updated_by
        FROM content
        "#,
    )
    .fetch_all(db)
    .await?;

    Ok(content)
}

/// For admin interfaces
pub async fn create(db: &PgPool, user_id: &str) -> Result<String, DbError> {
    let id = nanoid::nanoid!();
    let slug = nanoid::nanoid!();

    sqlx::query!(
        r#"
      INSERT INTO content (id, slug, title, markdown, meta_description, version, status, updated_at, updated_by)
      VALUES (
      $1,
      $2,
      $3,
      $4,
      NULL,
      1,
      'draft',
      NOW(),
      $5
      )
        "#,
        id,
        slug,
        "Default Title",
        "Default Content",
        user_id,
    )
    .execute(db)
    .await?;

    Ok(id)
}
