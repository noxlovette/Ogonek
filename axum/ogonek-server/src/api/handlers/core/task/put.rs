use crate::{
    AppState, Claims,
    api::{TASK_TAG, error::APIError},
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use ogonek_db::{
    core::task::{self, read_assignee, toggle},
    tracking::log_activity,
};
use ogonek_notifications::NotificationType;
use ogonek_types::{ActionType, ModelType, TaskWithFilesResponse};

/// Toggles completed/not completed on a task
#[utoipa::path(
    put,
    path = "/{id}",
    tag = TASK_TAG,
    params(
        ("id" = String, Path, description = "Task ID")
    ),
    responses(
        (status = 200, description = "Task with files retrieved", body = TaskWithFilesResponse),
        (status = 404, description = "Task not found"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn toggle_task(
    State(state): State<AppState>,
    Path(id): Path<String>,
    claims: Claims,
) -> Result<StatusCode, APIError> {
    let current_assignee = read_assignee(&state.db, &id, &claims.sub).await?;
    let should_notify = toggle(&state.db, &id, &claims.sub).await?;

    if should_notify {
        log_activity(
            &state.db,
            &claims.sub,
            &id,
            ModelType::Task,
            ActionType::Complete,
            current_assignee.as_deref(),
        )
        .await?;

        let task = task::read_by_id(&state.db, &id, &claims.sub).await?;

        state
            .notification_service
            .notify_teacher(
                &claims.sub,
                NotificationType::Completed {
                    task_title: task.title,
                    username: task.assignee_name,
                    task_id: task.id,
                },
            )
            .await?;
    }

    Ok(StatusCode::NO_CONTENT)
}
