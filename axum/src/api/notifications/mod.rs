use crate::api::error::APIError;
use crate::auth::Claims;
use crate::db::crud::notifications::device_tokens::*;
use crate::types::DeviceTokenPayload;
use crate::schema::AppState;
use axum::extract::{Json, State};
use axum::http::StatusCode;

#[utoipa::path(
    post,
    path = "",
    request_body = DeviceTokenPayload,
    responses(
        (status = 204, description = "Device Token Registered Successfully"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn register_device_token(
    State(state): State<AppState>,
    claims: Claims,
    Json(payload): Json<DeviceTokenPayload>,
) -> Result<StatusCode, APIError> {
    upsert(&state.db, &claims.sub, &payload).await?;
    Ok(StatusCode::NO_CONTENT)
}
