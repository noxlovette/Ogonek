use crate::api::error::APIError;
use crate::auth::jwt::Claims;
use crate::models::files::{FileMinimal, FileSmall};
use crate::models::meta::{CreationId, PaginatedResponse};
use crate::models::tasks::{
    TaskBodySmall, TaskBodyWithStudent, TaskCreateBody, TaskFileBind, TaskPaginationParams,
    TaskUpdate, TaskWithFilesResponse,
};
use crate::s3::post::delete_s3;
use crate::schema::AppState;
use axum::extract::{Json, Path, Query, State};
use hyper::StatusCode;

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
) -> Result<Json<TaskWithFilesResponse>, APIError> {
    // First fetch the task with assignee info
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
    .await?
    .ok_or_else(|| APIError::NotFound("Task not found".into()))?;

    // Then fetch the associated files
    let files = sqlx::query_as!(
        FileSmall,
        r#"
        SELECT
            f.id,
            f.name,
            f.mime_type,
            f.s3_key,
            f.size,
            f.owner_id
        FROM files f
        JOIN task_files tf ON f.id = tf.file_id
        WHERE tf.task_id = $1
        "#,
        id
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(TaskWithFilesResponse { task, files }))
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
        WHERE (t.assignee = ",
    );

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
    let mut count_query_builder =
        sqlx::QueryBuilder::new("SELECT COUNT(*) FROM tasks t WHERE (t.assignee = ");

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
) -> Result<Json<CreationId>, APIError> {
    let mut assignee = &claims.sub;

    if payload.assignee.is_some() {
        assignee = payload.assignee.as_ref().unwrap();
    }

    let id = sqlx::query_as!(
        CreationId,
        "INSERT INTO tasks (id, title, markdown, due_date, assignee, created_by)
         VALUES ($1, $2, $3, $4, $5, $6)
         RETURNING id
         ",
        nanoid::nanoid!(),
        payload.title,
        payload.markdown,
        payload.due_date,
        assignee,
        claims.sub,
    )
    .fetch_one(&state.db)
    .await?;

    Ok(Json(id))
}

pub async fn delete_task(
    State(state): State<AppState>,
    Path(id): Path<String>,
    claims: Claims,
) -> Result<StatusCode, APIError> {
    // Start by fetching the task to ensure it exists and user has permission
    let file_exists = sqlx::query!(
        r#"

        SELECT 1 as "exists!: bool"
        FROM tasks
        WHERE id = $1 AND (assignee = $2 OR created_by = $2)
        "#,
        id,
        claims.sub
    )
    .fetch_optional(&state.db)
    .await?
    .is_some();

    if !file_exists {
        return Err(APIError::NotFound("Task doesn't exist".into()));
    }

    let files = sqlx::query_as!(
        FileMinimal,
        r#"
        SELECT f.id, f.s3_key
        FROM files f
        JOIN task_files tf ON f.id = tf.file_id
        WHERE tf.task_id = $1
        "#,
        id
    )
    .fetch_all(&state.db)
    .await?;

    let file_ids: Vec<String> = files.iter().map(|f| f.id.clone()).collect();

    let mut tx = state.db.begin().await?;

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
        claims.sub
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    for file in files {
        if let Some(s3_key) = file.s3_key {
            if let Err(e) = delete_s3(&s3_key, &state).await {
                // Log error but don't fail the request since DB deletion was successful
                tracing::error!("Failed to delete file from S3: {}, error: {:?}", s3_key, e);
            }
        }
    }

    Ok(StatusCode::NO_CONTENT)
}

pub async fn update_task(
    State(state): State<AppState>,
    Path(id): Path<String>,
    claims: Claims,
    Json(payload): Json<TaskUpdate>,
) -> Result<StatusCode, APIError> {
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
        claims.sub,
        payload.title,
        payload.markdown,
        payload.priority,
        payload.completed,
        payload.due_date,
        payload.assignee,
    )
    .execute(&state.db)
    .await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn add_file_to_task(
    State(state): State<AppState>,
    Path(task_id): Path<String>,
    Json(payload): Json<TaskFileBind>,
) -> Result<StatusCode, APIError> {
    let mut tx = state.db.begin().await?;

    for file_id in payload.file_ids {
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

    Ok(StatusCode::CREATED)
}
