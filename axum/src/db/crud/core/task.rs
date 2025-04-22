use crate::db::error::DbError;
use crate::models::{
    CreationId, TaskCreate, TaskPaginationParams, TaskSmall, TaskUpdate, TaskWithStudent,
};
use sqlx::PgPool;

pub async fn find_all(
    db: &PgPool,
    user_id: &str,
    params: &TaskPaginationParams,
) -> Result<Vec<TaskWithStudent>, DbError> {
    let mut query_builder = sqlx::QueryBuilder::new(
        "SELECT
                t.id,
                t.title,
                t.markdown,
                t.priority,
                t.completed,
                t.due_date,
                t.file_path,
                t.assignee,
                t.created_by,
                t.created_at,
                t.updated_at,
                u.name as assignee_name
            FROM tasks t
            LEFT JOIN \"user\" u ON t.assignee = u.id
            WHERE (t.assignee = ",
    );

    query_builder.push_bind(user_id);
    query_builder.push(" OR t.created_by = ");
    query_builder.push_bind(user_id);
    query_builder.push(")");

    // Add search filter if provided
    if let Some(search) = &params.search {
        query_builder.push(" AND (t.title ILIKE ");
        query_builder.push_bind(format!("%{}%", search));
        query_builder.push(" OR t.markdown ILIKE ");
        query_builder.push_bind(format!("%{}%", search));
        query_builder.push(")");
    }

    // Add priority filter if provided
    if let Some(priority) = &params.priority {
        query_builder.push(" AND t.priority = ");
        query_builder.push_bind(priority);
    }

    // Add completed filter if provided
    if let Some(completed) = &params.completed {
        query_builder.push(" AND t.completed = ");
        query_builder.push_bind(completed);
    }

    if let Some(assignee) = &params.assignee {
        query_builder.push(" AND t.assignee = ");
        query_builder.push_bind(assignee);
    }

    // Add ordering - tasks typically ordered by due date
    query_builder.push(" ORDER BY t.due_date ASC NULLS LAST");

    // Add limit and offset
    query_builder.push(" LIMIT ");
    query_builder.push_bind(params.limit());
    query_builder.push(" OFFSET ");
    query_builder.push_bind(params.offset());

    // Execute the query
    let tasks = query_builder
        .build_query_as::<TaskWithStudent>()
        .fetch_all(db)
        .await?;

    Ok(tasks)
}

pub async fn find_by_id(db: &PgPool, id: &str, user_id: &str) -> Result<TaskWithStudent, DbError> {
    let task = sqlx::query_as!(
        TaskWithStudent,
        r#"
            SELECT
                t.id,
                t.title,
                t.markdown,
                t.priority,
                t.completed,
                t.due_date,
                t.assignee,
                t.created_by,
                t.created_at,
                t.updated_at,
                u.name as assignee_name
            FROM tasks t
            LEFT JOIN "user" u ON t.assignee = u.id
            WHERE t.id = $1
            AND (t.assignee = $2 OR t.created_by = $2)
            "#,
        id,
        user_id
    )
    .fetch_optional(db)
    .await?
    .ok_or_else(|| DbError::NotFound("Task not found".into()))?;

    Ok(task)
}

pub async fn create(
    db: &PgPool,
    task: &TaskCreate,
    user_id: &str,
    assignee: &str,
) -> Result<CreationId, DbError> {
    let id = sqlx::query_as!(
        CreationId,
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
pub async fn delete(
    db: &PgPool,
    id: &str,
    user_id: &str,
    file_ids: Vec<String>,
) -> Result<(), DbError> {
    let mut tx = db.begin().await?;

    if !file_ids.is_empty() {
        sqlx::query!(r#"DELETE FROM task_files WHERE task_id = $1"#, id)
            .execute(&mut *tx)
            .await?;

        sqlx::query!(r#"DELETE FROM files WHERE id = ANY($1)"#, &file_ids)
            .execute(&mut *tx)
            .await?;
    }

    sqlx::query!(
        r#"DELETE FROM tasks WHERE id = $1 AND (assignee = $2 OR created_by = $2)"#,
        id,
        user_id
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(())
}
pub async fn update(
    db: &PgPool,
    id: &str,
    user_id: &str,
    update: TaskUpdate,
) -> Result<(), DbError> {
    sqlx::query!(
        "UPDATE tasks
         SET
            title = COALESCE($3, title),
            markdown = COALESCE($4, markdown),
            priority = COALESCE($5, priority),
            completed = COALESCE($6, completed),
            due_date = COALESCE($7, due_date),
            assignee = COALESCE($8, assignee)
         WHERE id = $1 AND (assignee = $2 OR created_by = $2)",
        id,
        user_id,
        update.title,
        update.markdown,
        update.priority,
        update.completed,
        update.due_date,
        update.assignee,
    )
    .execute(db)
    .await?;

    Ok(())
}
pub async fn count(db: &PgPool, user_id: &str) -> Result<i64, DbError> {
    let count = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM tasks WHERE
            (created_by = $1 OR assignee = $1)
            ",
        user_id
    )
    .fetch_one(db)
    .await?;

    Ok(count.unwrap_or(0))
}

pub async fn fetch_recent(db: &PgPool, user_id: &str) -> Result<Vec<TaskSmall>, DbError> {
    let tasks = sqlx::query_as!(
        TaskSmall,
        r#"
        SELECT id,
            title,
            LEFT(markdown, 100) as "markdown!",
            completed,
            due_date
        FROM tasks
        WHERE (assignee = $1 OR created_by = $1)
        AND completed = false
        ORDER BY created_at DESC
        LIMIT 3
        "#,
        user_id
    )
    .fetch_all(db)
    .await?;

    Ok(tasks)
}

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
