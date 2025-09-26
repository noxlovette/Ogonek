use sqlx::PgPool;

use crate::{crud::core::file::check_file_exists, db::error::DbError, types::FileUpdate};

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
