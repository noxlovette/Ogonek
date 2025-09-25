use crate::api::TASK_TAG;
use crate::api::error::APIError;
use crate::auth::Claims;
use crate::crud::core::task::create_with_defaults;
use crate::db::crud::tracking::log_activity;
use crate::schema::AppState;
use crate::types::{ActionType, ModelType};
use axum::Json;
use axum::extract::State;

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
