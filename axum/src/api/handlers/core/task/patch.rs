use crate::api::TASK_TAG;
use crate::api::error::APIError;
use crate::auth::Claims;
use crate::crud::core::task::{self, update};
use crate::crud::tracking::seen;
use crate::db::crud::{core::task::find_assignee, tracking::log_activity};
use crate::schema::AppState;
use crate::services::messages::NotificationType;
use crate::types::{ActionType, ModelType, TaskUpdate};
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;

#[utoipa::path(
    patch,
    path = "/{id}",
    tag = TASK_TAG,
    params(
        ("id" = String, Path, description = "Task ID")
    ),
    request_body = TaskUpdate,
    responses(
        (status = 204, description = "Task updated successfully"),
        (status = 404, description = "Task not found"),
        (status = 401, description = "Unauthorized")
    )
)]
/// Updates the task
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
            let task = task::find_by_id(&state.db, &id, &claims.sub).await?;
            let _ = state
                .notification_service
                .notify_student(
                    &claims.sub,
                    &new_user,
                    NotificationType::TaskCreated {
                        task_title: task.title,
                        task_id: task.id,
                        due_date: task.due_date,
                    },
                )
                .await;
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
