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
        task::{self, read_by_id, read_public_one},
    },
    tracking::seen,
};
use ogonek_types::{
    ModelType, PaginatedResponse, PaginatedTasks, SortField, SortOrder, TaskPaginationParams,
    TaskPublicWithFiles, TaskSmall, TaskWithFilesResponse,
};

/// Retrieves a paginated list of tasks for the authenticated user
///
/// Returns tasks belonging to the user through assignment or ownership with pagination support.
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
    let (tasks, count) = task::read_all(&state.db, &claims.sub, &params).await?;

    let page = params.page();
    let per_page = params.limit();
    let total_pages = (count as f64 / per_page as f64).ceil() as i64;

    Ok(Json(PaginatedResponse {
        data: tasks,
        total_pages,
        page,
        count,
        per_page,
    }))
}

/// Retrieves a single task by ID with associated files
///
/// Returns task details with files and marks the task as seen by the user.
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

/// Retrieves a public task with files (no authentication required)
///
/// Returns task details with files for public access.
#[utoipa::path(
    get,
    path = "/task/{id}", tag = TASK_TAG,
    params(
        ("id" = String, Path, description = "Task ID")
    ),
    responses(
        (status = 200, description = "Task with files retrieved", body = TaskPublicWithFiles),
        (status = 404, description = "Task not found"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn fetch_task_public(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<TaskPublicWithFiles>, APIError> {
    let task = read_public_one(&state.db, &id).await?;
    let files = fetch_files_task(&state.db, &id).await?;

    Ok(Json(TaskPublicWithFiles { task, files }))
}
