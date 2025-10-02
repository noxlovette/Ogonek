use axum::{
    Json,
    extract::{Path, Query, State},
};

use crate::{
    AppState, Claims,
    api::{TASK_TAG, error::APIError},
};
use ogonek_db::{
    core::{
        file::fetch_files_task,
        task::{self, read_by_id},
    },
    tracking::seen,
};
use ogonek_types::{
    ModelType, PaginatedResponse, PaginatedTasks, SortField, SortOrder, TaskPaginationParams,
    TaskSmall, TaskWithFilesResponse,
};

/// Tasks belonging to a user (through assignment or direct ownership)
#[utoipa::path(
    get,
    path = "",
    tag = TASK_TAG,
    params(
        ("page" = Option<u32>, Query, description = "Page number"),
        ("per_page" = Option<u32>, Query, description = "Items per page"),
        ("search" = Option<String>, Query, description = "Search term"),
        ("assignee" = Option<String>, Query, description = "Filter by assignee"),
        ("completed" = Option<bool>, Query, description = "Filter by completion status"),
        ("sort_by" = Option<SortField>, Query),
        ("sort_order" = Option<SortOrder>, Query)
    ),
    responses(
        (status = 200, description = "Tasks retrieved successfully", body = PaginatedTasks),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_tasks(
    State(state): State<AppState>,
    claims: Claims,
    Query(params): Query<TaskPaginationParams>,
) -> Result<Json<PaginatedResponse<TaskSmall>>, APIError> {
    let tasks = task::read_all(&state.db, &claims.sub, &params).await?;

    Ok(Json(PaginatedResponse {
        data: tasks,
        page: params.page(),
        per_page: params.limit(),
    }))
}

/// One Task
#[utoipa::path(
    get,
    path = "/{id}", tag = TASK_TAG,
    params(
        ("id" = String, Path, description = "Task ID")
    ),
    responses(
        (status = 200, description = "Task with files retrieved", body = TaskWithFilesResponse),
        (status = 404, description = "Task not found"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn fetch_task(
    State(state): State<AppState>,
    Path(id): Path<String>,
    claims: Claims,
) -> Result<Json<TaskWithFilesResponse>, APIError> {
    let task = read_by_id(&state.db, &id, &claims.sub).await?;
    let files = fetch_files_task(&state.db, &id).await?;
    seen::mark_as_seen(&state.db, &claims.sub, &id, ModelType::Task).await?;

    Ok(Json(TaskWithFilesResponse { task, files }))
}
