use crate::{db::error::DbError, types::TaskCreate};
use sqlx::PgPool;
/// Creates a task, needs data to create FROM
pub async fn create(
    db: &PgPool,
    task: &TaskCreate,
    user_id: &str,
    assignee: &str,
) -> Result<String, DbError> {
    let id = sqlx::query_scalar!(
        "INSERT INTO tasks (id, title, markdown, due_date, assignee, created_by)
         VALUES ($1, $2, $3, $4, $5, $6)
         RETURNING id
         ",
        nanoid::nanoid!(),
        task.title,
        task.markdown,
        task.due_date,
        assignee,
        user_id,
    )
    .fetch_one(db)
    .await?;

    Ok(id)
}
/// Creates a task based on user preferences, faster – no JSON
pub async fn create_with_defaults(db: &PgPool, user_id: &str) -> Result<String, DbError> {
    let id = sqlx::query_scalar!(
        "INSERT INTO tasks (id, title, markdown, assignee, created_by)
         VALUES ($1, $2, $3, $4, $5 )
         RETURNING id
         ",
        nanoid::nanoid!(),
        "Default Title", // TODO: feed in preferred title
        "# Default markdown",
        user_id,
        user_id,
    )
    .fetch_one(db)
    .await?;

    Ok(id)
}
