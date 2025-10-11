use crate::{
    AppState, Claims,
    api::{TASK_TAG, error::APIError},
};
use axum::{Json, extract::State};
use ogonek_db::{core::task::create_with_defaults, tracking::log_activity};
use ogonek_types::{ActionType, ModelType};

/// Creates a new task with default values for the authenticated user
///
/// Generates a task with default settings and logs the creation activity.
#[utoipa::path(
    post,
    path = "",
    tag = TASK_TAG,
    responses(
        (status = 201, description = "Task created successfully", body = String),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn create_task(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<String>, APIError> {
    let id = create_with_defaults(&state.db, &claims.sub).await?;
    log_activity(
        &state.db,
        &claims.sub,
        &id,
        ModelType::Task,
        ActionType::Create,
        None,
    )
    .await?;

    Ok(Json(id))
}
