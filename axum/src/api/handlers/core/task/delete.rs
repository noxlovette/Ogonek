use crate::api::TASK_TAG;
use crate::api::error::APIError;
use crate::auth::Claims;
use crate::db::crud::{
    core::files::file::fetch_files_task,
    core::task::{delete, find_assignee},
    tracking::{delete_seen, log_activity},
};
use crate::schema::AppState;
use crate::types::{ActionType, ModelType};
use axum::extract::{Path, State};
use axum::http::StatusCode;

/// Deletes a task
#[utoipa::path(
    delete,
    path = "/{id}",
    tag = TASK_TAG,
    params(
        ("id" = String, Path, description = "Task ID")
    ),
    responses(
        (status = 204, description = "Task deleted successfully"),
        (status = 404, description = "Task not found"),
        (status = 401, description = "Unauthorized")
    )
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
        if let Some(s3_key) = file.s3_key
            && let Err(e) = state.s3.delete_s3(&s3_key).await
        {
            tracing::error!("Failed to delete file from S3: {}, error: {:?}", s3_key, e);
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
