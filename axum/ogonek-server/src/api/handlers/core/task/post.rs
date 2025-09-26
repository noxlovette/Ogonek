use crate::{
    api::{TASK_TAG, error::APIError},
    Claims,
    AppState,
};
use ogonek_db::{
    core::task::create_with_defaults,
    tracking::log_activity,
};
use ogonek_types::{ActionType, ModelType};
use axum::{Json, extract::State};

/// Creates a new task
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
