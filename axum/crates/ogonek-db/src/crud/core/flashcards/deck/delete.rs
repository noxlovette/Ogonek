use crate::DbError;

/// Deletes a deck
pub async fn delete(
    db: impl sqlx::Executor<'_, Database = sqlx::Postgres>,
    deck_id: &str,
    user_id: &str,
) -> Result<(), DbError> {
    sqlx::query!(
        r#"
        DELETE FROM decks
        WHERE id = $1 AND created_by = $2
        "#,
        deck_id,
        user_id
    )
    .execute(db)
    .await?;

    Ok(())
}
