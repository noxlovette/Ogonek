use crate::api::TASK_TAG;
use crate::api::error::APIError;
use crate::auth::Claims;
use crate::crud::core::task::{self, toggle};
use crate::db::crud::{core::task::find_assignee, tracking::log_activity};
use crate::schema::AppState;
use crate::services::messages::NotificationType;
use crate::types::{ActionType, ModelType, TaskWithFilesResponse};
use axum::extract::{Path, State};
use axum::http::StatusCode;

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
    let current_assignee = find_assignee(&state.db, &id, &claims.sub).await?;
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

        let task = task::find_by_id(&state.db, &id, &claims.sub).await?;

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
