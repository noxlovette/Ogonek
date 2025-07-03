use crate::api::error::APIError;
use crate::auth::Claims;
use crate::db::crud::core;
use crate::db::crud::core::task::{
    add_files, count, create, delete, fetch_recent, find_all, find_assignee, find_by_id, update,
};
use crate::db::crud::files::file::fetch_files_task;

use crate::models::meta::{CreationId, PaginatedResponse};
use crate::models::tasks::{
    TaskCreate, TaskFileBind, TaskPaginationParams, TaskSmall, TaskUpdate, TaskWithFilesResponse,
    TaskWithStudent,
};
use crate::s3::post::delete_s3;
use crate::schema::AppState;
use axum::extract::{Json, Path, Query, State};
use hyper::StatusCode;

pub async fn fetch_recent_tasks(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<Vec<TaskSmall>>, APIError> {
    let tasks = fetch_recent(&state.db, &claims.sub).await?;

    Ok(Json(tasks))
}

pub async fn fetch_task(
    State(state): State<AppState>,
    Path(id): Path<String>,
    claims: Claims,
) -> Result<Json<TaskWithFilesResponse>, APIError> {
    let task = find_by_id(&state.db, &id, &claims.sub).await?;
    let files = fetch_files_task(&state.db, &id).await?;
    core::seen::mark_as_seen(&state.db, &claims.sub, &id, core::seen::ModelType::Task).await?;

    Ok(Json(TaskWithFilesResponse { task, files }))
}

pub async fn list_tasks(
    State(state): State<AppState>,
    claims: Claims,
    Query(params): Query<TaskPaginationParams>,
) -> Result<Json<PaginatedResponse<TaskWithStudent>>, APIError> {
    let tasks = find_all(&state.db, &claims.sub, &params).await?;
    let count = count(&state.db, &claims.sub).await?;

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
    Json(payload): Json<TaskCreate>,
) -> Result<Json<CreationId>, APIError> {
    let mut assignee = &claims.sub;

    if payload.assignee.is_some() {
        assignee = payload.assignee.as_ref().unwrap();
    }

    let id = create(&state.db, &payload, &claims.sub, assignee).await?;

    Ok(Json(id))
}

pub async fn delete_task(
    State(state): State<AppState>,
    Path(id): Path<String>,
    claims: Claims,
) -> Result<StatusCode, APIError> {
    let files = fetch_files_task(&state.db, &id).await?;

    let file_ids: Vec<String> = files.iter().map(|f| f.id.clone()).collect();

    delete(&state.db, &id, &claims.sub, file_ids).await?;

    for file in files {
        if let Some(s3_key) = file.s3_key {
            if let Err(e) = delete_s3(&s3_key, &state).await {
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
    // fetch assignee before update
    let current_assignee = find_assignee(&state.db, &id, &claims.sub).await?;

    // update the task
    update(&state.db, &id, &claims.sub, &payload).await?;

    // check if assignee changed
    let new_assignee = payload.assignee.clone();

    if new_assignee != current_assignee {
        // remove seen record for old assignee
        if let Some(old_user) = current_assignee {
            core::seen::delete_seen(&state.db, &old_user, &id, core::seen::ModelType::Task).await?;
        }

        // insert unseen for new assignee
        if let Some(new_user) = new_assignee {
            core::seen::insert_as_unseen(&state.db, &new_user, &id, core::seen::ModelType::Task)
                .await?;
        }
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
