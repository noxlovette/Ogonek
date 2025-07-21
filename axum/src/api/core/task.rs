use crate::api::error::APIError;
use crate::auth::Claims;
use crate::db::crud::core::task::{
    add_files, count, create, delete, find_all, find_assignee, find_by_id, toggle, update,
};
use crate::db::crud::files::file::fetch_files_task;

use crate::db::crud::tracking::activity::log_activity;
use crate::db::crud::tracking::seen::delete_seen;
use crate::db::crud::tracking::{ActionType, ModelType, seen};
use crate::models::TaskFull;
use crate::models::meta::{CreationId, PaginatedResponse};
use crate::models::tasks::{
    TaskCreate, TaskFileBind, TaskPaginationParams, TaskSmall, TaskUpdate, TaskWithFilesResponse,
};
use crate::s3::post::delete_s3;
use crate::schema::AppState;
use axum::extract::{Json, Path, Query, State};
use hyper::StatusCode;

/// One Task
#[utoipa::path(
    get,
    path = "/task/t/{id}",
    params(
        ("id" = String, Path, description = "Task ID")
    ),
    responses(
        (status = 200, description = "Task with files retrieved", body = TaskWithFilesResponse),
        (status = 404, description = "Task not found"),
        (status = 401, description = "Unauthorized")
    ),
    security(("api_key" = []))
)]
pub async fn fetch_task(
    State(state): State<AppState>,
    Path(id): Path<String>,
    claims: Claims,
) -> Result<Json<TaskWithFilesResponse>, APIError> {
    let task = find_by_id(&state.db, &id, &claims.sub).await?;
    let files = fetch_files_task(&state.db, &id).await?;
    seen::mark_as_seen(&state.db, &claims.sub, &id, ModelType::Task).await?;

    Ok(Json(TaskWithFilesResponse { task, files }))
}
#[utoipa::path(
    get,
    path = "/task",
    params(
        ("page" = Option<u32>, Query, description = "Page number"),
        ("per_page" = Option<u32>, Query, description = "Items per page"),
        ("search" = Option<String>, Query, description = "Search term"),
        ("assignee" = Option<String>, Query, description = "Filter by assignee"),
        ("completed" = Option<bool>, Query, description = "Filter by completion status"),
        ("priority" = Option<i32>, Query, description = "Filter by priority")
    ),
    responses(
        (status = 200, description = "Tasks retrieved successfully", body = PaginatedResponse<TaskFull>),
        (status = 401, description = "Unauthorized")
    ),
    security(("api_key" = []))
)]
pub async fn list_tasks(
    State(state): State<AppState>,
    claims: Claims,
    Query(params): Query<TaskPaginationParams>,
) -> Result<Json<PaginatedResponse<TaskSmall>>, APIError> {
    let tasks = find_all(&state.db, &claims.sub, &params).await?;
    let count = count(&state.db, &claims.sub).await?;

    Ok(Json(PaginatedResponse {
        data: tasks,
        total: count,
        page: params.page(),
        per_page: params.limit(),
    }))
}
#[utoipa::path(
    post,
    path = "/task",
    request_body = TaskCreate,
    responses(
        (status = 201, description = "Task created successfully", body = CreationId),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized")
    ),
    security(("api_key" = []))
)]
pub async fn create_task(
    State(state): State<AppState>,
    claims: Claims,
    Json(payload): Json<TaskCreate>,
) -> Result<Json<CreationId>, APIError> {
    let mut assignee = &claims.sub;

    if payload.assignee.is_some() {
        assignee = payload.assignee.as_ref().unwrap();
    }

    let id = create(&state.db, &payload, &claims.sub, assignee).await?;

    log_activity(
        &state.db,
        &claims.sub,
        &id.id,
        ModelType::Task,
        ActionType::Create,
        None,
    )
    .await?;

    Ok(Json(id))
}
#[utoipa::path(
    delete,
    path = "/task/t/{id}",
    params(
        ("id" = String, Path, description = "Task ID")
    ),
    responses(
        (status = 204, description = "Task deleted successfully"),
        (status = 404, description = "Task not found"),
        (status = 401, description = "Unauthorized")
    ),
    security(("api_key" = []))
)]
pub async fn delete_task(
    State(state): State<AppState>,
    Path(id): Path<String>,
    claims: Claims,
) -> Result<StatusCode, APIError> {
    let files = fetch_files_task(&state.db, &id).await?;

    let file_ids: Vec<String> = files.iter().map(|f| f.id.clone()).collect();
    let assignee = find_assignee(&state.db, &id, &claims.sub).await?;

    delete(&state.db, &id, &claims.sub, file_ids).await?;

    for file in files {
        if let Some(s3_key) = file.s3_key {
            if let Err(e) = delete_s3(&s3_key, &state).await {
                tracing::error!("Failed to delete file from S3: {}, error: {:?}", s3_key, e);
            }
        }
    }

    if let Some(user) = assignee {
        delete_seen(&state.db, &user, &id, ModelType::Task).await?;

        // log deletion activity
        log_activity(
            &state.db,
            &claims.sub,
            &id,
            ModelType::Task,
            ActionType::Delete,
            Some(&user),
        )
        .await?;
    }
    Ok(StatusCode::NO_CONTENT)
}

pub async fn toggle_task(
    State(state): State<AppState>,
    Path(id): Path<String>,
    claims: Claims,
) -> Result<StatusCode, APIError> {
    let current_assignee = find_assignee(&state.db, &id, &claims.sub).await?;
    toggle(&state.db, &id, &claims.sub).await?;

    log_activity(
        &state.db,
        &claims.sub,
        &id,
        ModelType::Task,
        ActionType::Complete,
        current_assignee.as_deref(),
    )
    .await?;
    Ok(StatusCode::NO_CONTENT)
}
#[utoipa::path(
    patch,
    path = "/task/t/{id}",
    params(
        ("id" = String, Path, description = "Task ID")
    ),
    request_body = TaskUpdate,
    responses(
        (status = 204, description = "Task updated successfully"),
        (status = 404, description = "Task not found"),
        (status = 401, description = "Unauthorized")
    ),
    security(("api_key" = []))
)]
pub async fn update_task(
    State(state): State<AppState>,
    Path(id): Path<String>,
    claims: Claims,
    Json(payload): Json<TaskUpdate>,
) -> Result<StatusCode, APIError> {
    // fetch assignee before update
    let current_assignee = find_assignee(&state.db, &id, &claims.sub).await?;

    // update the task
    update(&state.db, &id, &claims.sub, &payload).await?;

    // check if assignee changed
    let new_assignee = payload.assignee.clone();

    if new_assignee != current_assignee {
        // remove seen record for old assignee
        if let Some(old_user) = current_assignee {
            seen::delete_seen(&state.db, &old_user, &id, ModelType::Task).await?;
            // treat as deletion
            log_activity(
                &state.db,
                &old_user,
                &id,
                ModelType::Task,
                ActionType::Delete,
                None,
            )
            .await?;
        }

        // insert unseen for new assignee
        if let Some(new_user) = new_assignee {
            seen::insert_as_unseen(&state.db, &new_user, &id, ModelType::Task).await?;

            // treat as creation
            log_activity(
                &state.db,
                &new_user,
                &id,
                ModelType::Task,
                ActionType::Create,
                None,
            )
            .await?;
        }
    } else if let Some(assignee) = current_assignee {
        // treat as update
        log_activity(
            &state.db,
            &claims.sub,
            &id,
            ModelType::Task,
            ActionType::Update,
            Some(&assignee),
        )
        .await?;
    }
    Ok(StatusCode::NO_CONTENT)
}

pub async fn add_file_to_task(
    State(state): State<AppState>,
    Path(task_id): Path<String>,
    Json(payload): Json<TaskFileBind>,
) -> Result<StatusCode, APIError> {
    add_files(&state.db, &task_id, payload.file_ids).await?;

    Ok(StatusCode::CREATED)
}
