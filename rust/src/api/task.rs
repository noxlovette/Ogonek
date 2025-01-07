use crate::auth::jwt::Claims;
use crate::db::error::DbError;
use crate::db::init::AppState;
use crate::models::tasks::{TaskBody, TaskCreateBody, TaskUpdate};
use axum::extract::{Json, Path, State};

pub async fn fetch_task(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Option<TaskBody>>, DbError> {
    let task = sqlx::query_as!(TaskBody, "SELECT * FROM tasks WHERE id = $1", id)
        .fetch_optional(&state.db)
        .await?;
    Ok(Json(task))
}

pub async fn list_tasks(State(state): State<AppState>) -> Result<Json<Vec<TaskBody>>, DbError> {
    let tasks = sqlx::query_as!(TaskBody, "SELECT * FROM tasks")
        .fetch_all(&state.db)
        .await?;
    Ok(Json(tasks))
}

pub async fn create_task(
    State(state): State<AppState>,
    claims: Claims,
    Json(payload): Json<TaskCreateBody>,
) -> Result<Json<TaskBody>, DbError> {
    let task = sqlx::query_as!(
        TaskBody,
        "INSERT INTO tasks (id, title, markdown, due_date, assignee, created_by)
         VALUES ($1, $2, $3, $4, $5, $6)
         RETURNING *",
        nanoid::nanoid!(),
        payload.title,
        payload.markdown,
        payload.due_date,
        payload.assignee,
        claims.sub,
    )
    .fetch_one(&state.db)
    .await?;
    Ok(Json(task))
}

pub async fn delete_task(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<TaskBody>, DbError> {
    let task = sqlx::query_as!(TaskBody, "DELETE FROM tasks WHERE id = $1 RETURNING *", id)
        .fetch_optional(&state.db)
        .await?
        .ok_or(DbError::NotFound)?;
    Ok(Json(task))
}

pub async fn update_task(
    State(state): State<AppState>,
    Path(id): Path<String>,
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
            assignee = COALESCE($6, assignee)
         WHERE id = $7
         RETURNING *",
        payload.title,
        payload.markdown,
        payload.priority,
        payload.completed,
        payload.due_date,
        payload.assignee,
        id
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or(DbError::NotFound)?;
    Ok(Json(task))
}
