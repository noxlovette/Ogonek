use crate::auth::jwt::Claims;
use super::error::APIError;
use crate::db::init::AppState;
use crate::models::tasks::{TaskBody, TaskBodySmall, TaskBodyWithStudent, TaskCreateBody, TaskUpdate, TaskPaginationParams};
use crate::models::meta::PaginatedResponse;
use axum::extract::{Json, Path, State, Query};

pub async fn fetch_recent_tasks(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<Vec<TaskBodySmall>>, APIError> {
    let tasks = sqlx::query_as!(
        TaskBodySmall,
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
        claims.sub
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(tasks))
}

pub async fn fetch_task(
    State(state): State<AppState>,
    Path(id): Path<String>,
    claims: Claims,
) -> Result<Json<Option<TaskBodyWithStudent>>, APIError> {
    let task = sqlx::query_as!(
        TaskBodyWithStudent,
        r#"
        SELECT 
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
        LEFT JOIN "user" u ON t.assignee = u.id
        WHERE t.id = $1 
        AND (t.assignee = $2 OR t.created_by = $2)
        "#,
        id,
        claims.sub
    )
    .fetch_optional(&state.db)
    .await?;
    Ok(Json(task))
}

pub async fn list_tasks(
    State(state): State<AppState>,
    claims: Claims,
    Query(params): Query<TaskPaginationParams>,
) -> Result<Json<PaginatedResponse<TaskBodyWithStudent>>, APIError> {
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
        WHERE (t.assignee = ");
    
    query_builder.push_bind(&claims.sub);
    query_builder.push(" OR t.created_by = ");
    query_builder.push_bind(&claims.sub);
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
        .build_query_as::<TaskBodyWithStudent>()
        .fetch_all(&state.db)
        .await?;
    
    // Count query - build a similar query without LIMIT/OFFSET but with COUNT
    let mut count_query_builder = sqlx::QueryBuilder::new(
        "SELECT COUNT(*) FROM tasks t WHERE (t.assignee = ");
    
    count_query_builder.push_bind(&claims.sub);
    count_query_builder.push(" OR t.created_by = ");
    count_query_builder.push_bind(&claims.sub);
    count_query_builder.push(")");
    
    // Add the same filters as the main query
    if let Some(search) = &params.search {
        count_query_builder.push(" AND (t.title ILIKE ");
        count_query_builder.push_bind(format!("%{}%", search));
        count_query_builder.push(" OR t.markdown ILIKE ");
        count_query_builder.push_bind(format!("%{}%", search));
        count_query_builder.push(")");
    }
    
    if let Some(priority) = &params.priority {
        count_query_builder.push(" AND t.priority = ");
        count_query_builder.push_bind(priority);
    }
    
    if let Some(completed) = &params.completed {
        count_query_builder.push(" AND t.completed = ");
        count_query_builder.push_bind(completed);
    }

    if let Some(assignee) = &params.assignee {
        count_query_builder.push(" AND t.assignee = ");
        count_query_builder.push_bind(assignee);
    }
    
    let count: i64 = count_query_builder
        .build_query_scalar()
        .fetch_one(&state.db)
        .await?;
    
    // Return paginated response
    Ok(Json(PaginatedResponse {
        data: tasks,
        total: count,
        page: params.page(),
        per_page: params.limit(),
    }))
}


pub async fn create_task(
    State(state): State<AppState>,
    claims: Claims,
    Json(payload): Json<TaskCreateBody>,
) -> Result<Json<TaskBody>, APIError> {
    let mut assignee = &claims.sub;

    if payload.assignee.is_some() {
        assignee = payload.assignee.as_ref().unwrap();
    }

    let task = sqlx::query_as!(
        TaskBody,
        "INSERT INTO tasks (id, title, markdown, due_date, assignee, created_by)
         VALUES ($1, $2, $3, $4, $5, $6)
         RETURNING *",
        nanoid::nanoid!(),
        payload.title,
        payload.markdown,
        payload.due_date,
        assignee,
        claims.sub,
    )
    .fetch_one(&state.db)
    .await?;
    Ok(Json(task))
}

pub async fn delete_task(
    State(state): State<AppState>,
    Path(id): Path<String>,
    claims: Claims,
) -> Result<Json<TaskBody>, APIError> {
    let task = sqlx::query_as!(
        TaskBody,
        "DELETE FROM tasks WHERE id = $1 AND (assignee = $2 OR created_by = $2) RETURNING *",
        id,
        claims.sub
    )
    .fetch_optional(&state.db)
    .await
    .map_err(APIError::Database)?
    .ok_or_else(|| APIError::NotFound("Task not found".into()))?;

    Ok(Json(task))
}

pub async fn update_task(
    State(state): State<AppState>,
    Path(id): Path<String>, 
    claims: Claims,
    Json(payload): Json<TaskUpdate>,
) -> Result<Json<TaskBody>, APIError> {
    let task = sqlx::query_as!(
        TaskBody,
        "UPDATE tasks
         SET
            title = COALESCE($1, title),
            markdown = COALESCE($2, markdown),
            priority = COALESCE($3, priority),
            completed = COALESCE($4, completed),
            due_date = COALESCE($5, due_date),
            assignee = COALESCE($6, assignee),
            file_path = COALESCE($7, file_path)
         WHERE id = $8 AND (assignee = $9 OR created_by = $9)
         RETURNING *",
        payload.title,
        payload.markdown,
        payload.priority,
        payload.completed,
        payload.due_date,
        payload.assignee,
        payload.file_path,
        id,
        claims.sub
    )
    .fetch_optional(&state.db)
    .await
    .map_err(APIError::Database)?
    .ok_or_else(|| APIError::NotFound("Task not found".into()))?;

    Ok(Json(task))
}
