use crate::DbError;

use ogonek_types::TaskUpdate;
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
    // Let the DB constraint handle the validation
    sqlx::query!(
        "UPDATE tasks
         SET
            title = COALESCE($3, title),
            markdown = COALESCE($4, markdown),
            due_date = COALESCE($5, due_date),
            assignee = COALESCE($6, assignee),
            visibility = COALESCE($7, visibility)
         WHERE id = $1 AND (assignee = $2 OR created_by = $2)",
        id,
        user_id,
        update.title,
        update.markdown,
        update.due_date,
        update.assignee,
        update.visibility.as_ref().map(|v| v.to_string())
    )
    .execute(db)
    .await?;
    Ok(())
}
