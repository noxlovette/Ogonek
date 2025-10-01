use crate::DbError;

use ogonek_types::{PDFData, TaskFull, TaskPaginationParams, TaskSmall};
use sqlx::PgPool;
/// Mini-tasks
pub async fn read_all(
    db: &PgPool,
    user_id: &str,
    params: &TaskPaginationParams,
) -> Result<Vec<TaskSmall>, DbError> {
    let mut query_builder = sqlx::QueryBuilder::new(
        r#"SELECT
                t.id,
                t.title,
                t.priority,
                t.completed,
                t.due_date,
                u.name as assignee_name,
                COALESCE(s.seen_at IS NOT NULL, TRUE) as seen
            FROM tasks t
            LEFT JOIN "user" u ON t.assignee = u.id
            LEFT JOIN seen_status s ON s.model_id = t.id AND s.user_id = "#,
    );
    query_builder.push_bind(user_id);
    query_builder.push(
        r#"
            WHERE (t.assignee = "#,
    );
    query_builder.push_bind(user_id);
    query_builder.push(" OR t.created_by = ");
    query_builder.push_bind(user_id);
    query_builder.push(")");

    // Add search filter if provided
    if let Some(search) = &params.search {
        query_builder.push(" AND (t.title ILIKE ");
        query_builder.push_bind(format!("%{search}%"));
        query_builder.push(" OR t.markdown ILIKE ");
        query_builder.push_bind(format!("%{search}%"));
        query_builder.push(")");
    }

    // If completed=false (default), show only incomplete tasks
    // If completed=true, show all tasks (both complete and incomplete mixed)
    if let Some(completed) = &params.completed {
        if !completed {
            query_builder.push(" AND t.completed = false");
        }
        // If completed=true, don't add any filter (show all)
    }

    if let Some(assignee) = &params.assignee {
        query_builder.push(" AND t.assignee = ");
        query_builder.push_bind(assignee);
    }

    // Add ordering - incomplete tasks first, then by due date
    query_builder.push(" ORDER BY t.completed ASC, t.due_date ASC NULLS LAST");

    // Add limit and offset
    query_builder.push(" LIMIT ");
    query_builder.push_bind(params.limit());
    query_builder.push(" OFFSET ");
    query_builder.push_bind(params.offset());

    // Execute the query
    let tasks = query_builder
        .build_query_as::<TaskSmall>()
        .fetch_all(db)
        .await?;
    Ok(tasks)
}

/// Returns the full Task with all fields
pub async fn read_by_id(db: &PgPool, id: &str, user_id: &str) -> Result<TaskFull, DbError> {
    let task = sqlx::query_as!(
        TaskFull,
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
                u.name as "assignee_name?"
            FROM tasks t
            LEFT JOIN "user" u ON t.assignee = u.id
            WHERE t.id = $1
            AND (t.assignee = $2 OR t.created_by = $2)
            "#,
        id,
        user_id,
    )
    .fetch_one(db)
    .await?;

    Ok(task)
}

/// Finds the assignee for the task
pub async fn read_assignee(
    db: &PgPool,
    lesson_id: &str,
    user_id: &str,
) -> Result<Option<String>, DbError> {
    let assignee = sqlx::query_scalar!(
        r#"
        SELECT assignee
        FROM tasks
        WHERE id = $1
        AND (assignee = $2 OR created_by = $2)
        "#,
        lesson_id,
        user_id
    )
    .fetch_optional(db)
    .await?;

    Ok(assignee.flatten())
}

/// Finds the markdown and the title
pub async fn read_for_pdf(db: &PgPool, task_id: &str, user_id: &str) -> Result<PDFData, DbError> {
    let data = sqlx::query_as!(
        PDFData,
        r#"
        SELECT markdown, title
        FROM tasks
        WHERE id = $1
        AND (assignee = $2 OR created_by = $2)
        "#,
        task_id,
        user_id
    )
    .fetch_one(db)
    .await?;

    Ok(data)
}

pub async fn read_old_tasks(db: &PgPool) -> Result<Vec<String>, DbError> {
    let tasks = sqlx::query_scalar!(
        r#"
        SELECT id
        FROM tasks
        WHERE created_at < NOW() - INTERVAL '1 month'
        AND completed = true;
        "#
    )
    .fetch_all(db)
    .await?;

    Ok(tasks)
}

pub async fn read_count(db: &PgPool, user_id: &str) -> Result<i64, DbError> {
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

pub async fn read_recent(db: &PgPool, user_id: &str) -> Result<Vec<TaskSmall>, DbError> {
    let tasks = sqlx::query_as!(
        TaskSmall,
        r#"
        SELECT
                t.id,
                t.title,
                t.priority,
                t.completed,
                t.due_date,
                u.name as "assignee_name?",
               COALESCE(s.seen_at IS NOT NULL, TRUE) as seen
            FROM tasks t
            LEFT JOIN "user" u ON t.assignee = u.id
            LEFT JOIN seen_status s ON s.model_id = t.id AND s.user_id = $1
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
