use crate::{db::error::DbError, types::TaskUpdate};
use sqlx::PgPool;
/// Finds the assignee for the task
pub async fn toggle(db: &PgPool, task_id: &str, user_id: &str) -> Result<bool, DbError> {
    let completed = sqlx::query_scalar!(
        r#"
        SELECT completed
        FROM tasks
        WHERE id = $1
        AND (assignee = $2 OR created_by = $2)
        "#,
        task_id,
        user_id
    )
    .fetch_one(db)
    .await?;

    sqlx::query!(
        r#"
       UPDATE tasks
       SET
        completed = $3
         WHERE id = $1 AND (assignee = $2 OR created_by = $2)
       "#,
        task_id,
        user_id,
        !completed
    )
    .execute(db)
    .await?;

    Ok(!completed)
}

/// Updates the task and inserts associated files
pub async fn update(
    db: &PgPool,
    id: &str,
    user_id: &str,
    update: &TaskUpdate,
) -> Result<(), DbError> {
    sqlx::query!(
        "UPDATE tasks
         SET
            title = COALESCE($3, title),
            markdown = COALESCE($4, markdown),
            completed = COALESCE($5, completed),
            due_date = COALESCE($6, due_date),
            assignee = COALESCE($7, assignee)
         WHERE id = $1 AND (assignee = $2 OR created_by = $2)",
        id,
        user_id,
        update.title,
        update.markdown,
        update.completed,
        update.due_date,
        update.assignee,
    )
    .execute(db)
    .await?;

    Ok(())
}
