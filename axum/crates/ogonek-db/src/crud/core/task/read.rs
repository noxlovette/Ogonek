use crate::DbError;

use ogonek_types::{PDFData, SortField, TaskFull, TaskPaginationParams, TaskSmall};
use sqlx::PgPool;
/// Mini-tasks
pub async fn read_all(
    db: &PgPool,
    user_id: &str,
    params: &TaskPaginationParams,
) -> Result<(Vec<TaskSmall>, i64), DbError> {
    let mut query_builder = sqlx::QueryBuilder::new(
        r#"SELECT
                t.id,
                t.title,
                t.priority,
                t.completed,
                t.visibility,
                t.due_date,
                t.created_at,
                t.updated_at,
                u.name as assignee_name,
                COALESCE(s.seen_at IS NOT NULL, TRUE) as seen
            FROM tasks t
            LEFT JOIN "user" u ON t.assignee = u.id
            LEFT JOIN seen_status s ON s.model_id = t.id AND s.user_id = "#,
    );
    query_builder.push_bind(user_id);

    // Base WHERE clause - user must be assignee or creator
    query_builder.push(" WHERE (t.assignee = ");
    query_builder.push_bind(user_id);
    query_builder.push(" OR t.created_by = ");
    query_builder.push_bind(user_id);
    query_builder.push(")");

    // Search filter
    if let Some(search) = &params.search {
        query_builder.push(" AND (t.title ILIKE ");
        query_builder.push_bind(format!("%{search}%"));
        query_builder.push(" OR t.markdown ILIKE ");
        query_builder.push_bind(format!("%{search}%"));
        query_builder.push(")");
    }

    // Completed filter
    match params.completed {
        Some(false) | None => {
            // Show only incomplete tasks
            query_builder.push(" AND t.completed = false");
        }
        Some(true) => {
            // Show both completed and incomplete tasks (no filter)
        }
    }

    // Assignee filter
    if let Some(assignee) = &params.assignee {
        query_builder.push(" AND t.assignee = ");
        query_builder.push_bind(assignee);
    }

    // HERE'S THE KEY CHANGE: Dynamic sorting
    query_builder.push(" ORDER BY ");

    // Special case: Always prioritize incomplete tasks if not explicitly sorting by completion
    match params.sort_by {
        SortField::Title | SortField::CreatedAt | SortField::UpdatedAt => {
            // Incomplete tasks first, THEN apply user's sort
            query_builder.push("t.completed ASC, ");
        }
        SortField::DueDate => {
            // For due date, incomplete tasks first is implicit via NULLS handling
            query_builder.push("t.completed ASC, ");
        }
    }

    // Apply user's sort preference
    query_builder.push(params.sort_by.to_task_column());
    query_builder.push(" ");
    query_builder.push(params.sort_order.to_sql());

    // Handle NULLs for due_date specifically
    if matches!(params.sort_by, SortField::DueDate) {
        query_builder.push(" NULLS LAST");
    }

    // Pagination
    query_builder.push(" LIMIT ");
    query_builder.push_bind(params.limit());
    query_builder.push(" OFFSET ");
    query_builder.push_bind(params.offset());

    let tasks = query_builder
        .build_query_as::<TaskSmall>()
        .fetch_all(db)
        .await?;
    // Build count query with same filters
    let mut count_query =
        sqlx::QueryBuilder::new(r#"SELECT COUNT(*) FROM tasks t WHERE (t.assignee = "#);
    count_query.push_bind(user_id);
    count_query.push(" OR t.created_by = ");
    count_query.push_bind(user_id);
    count_query.push(")");

    if let Some(search) = &params.search {
        count_query.push(" AND (t.title ILIKE ");
        count_query.push_bind(format!("%{search}%"));
        count_query.push(" OR t.markdown ILIKE ");
        count_query.push_bind(format!("%{search}%"));
        count_query.push(")");
    }

    match params.completed {
        Some(false) | None => {
            count_query.push(" AND t.completed = false");
        }
        Some(true) => {
            // Show both completed and incomplete tasks (no filter)
        }
    }

    if let Some(assignee) = &params.assignee {
        count_query.push(" AND t.assignee = ");
        count_query.push_bind(assignee);
    }

    let total: (i64,) = count_query.build_query_as().fetch_one(db).await?;

    Ok((tasks, total.0))
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
                t.visibility,
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
