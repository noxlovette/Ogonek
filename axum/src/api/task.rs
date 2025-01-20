use crate::auth::jwt::Claims;
use crate::db::error::DbError;
use crate::db::init::AppState;
use crate::models::tasks::{TaskBody, TaskBodyWithStudent, TaskCreateBody, TaskUpdate};
use axum::extract::{Json, Path, State};

pub async fn fetch_task(
    State(state): State<AppState>,
    Path(id): Path<String>,
    claims: Claims,
) -> Result<Json<Option<TaskBodyWithStudent>>, DbError> {
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
) -> Result<Json<Vec<TaskBodyWithStudent>>, DbError> {
    let tasks = sqlx::query_as!(
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
        WHERE (t.assignee = $1 OR t.created_by = $1)
        "#,
        claims.sub
    )
    .fetch_all(&state.db)
    .await?;
    Ok(Json(tasks))
}

pub async fn create_task(
    State(state): State<AppState>,
    claims: Claims,
    Json(payload): Json<TaskCreateBody>,
) -> Result<Json<TaskBody>, DbError> {
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
) -> Result<Json<TaskBody>, DbError> {
    let task = sqlx::query_as!(
        TaskBody,
        "DELETE FROM tasks WHERE id = $1 AND (assignee = $2 OR created_by = $2) RETURNING *",
        id,
        claims.sub
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or(DbError::NotFound)?;
    Ok(Json(task))
}

pub async fn update_task(
    State(state): State<AppState>,
    Path(id): Path<String>, 
    claims: Claims,
    Json(payload): Json<TaskUpdate>,
) -> Result<Json<TaskBody>, DbError> {
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
    .await?
    .ok_or(DbError::NotFound)?;
    Ok(Json(task))
}
