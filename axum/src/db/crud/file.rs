use crate::{db::error::DbError, schema::AppState};

pub async fn check_file_exists(
    state: &AppState,
    file_id: &str,
    user_id: &str,
) -> Result<(), DbError> {
    let file_exists = sqlx::query!(
        r#"
        SELECT 1 as "exists!: bool" FROM files
        WHERE id = $1 AND owner_id = $2 AND upload_status = 'pending'
        "#,
        file_id,
        user_id
    )
    .fetch_optional(&state.db)
    .await?
    .is_some();

    if !file_exists {
        return Err(DbError::NotFound("File not found".into()));
    }

    Ok(())
}
