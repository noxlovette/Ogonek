use crate::{AppState, Claims, api::error::APIError};
use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use ogonek_db::{core::account::user, notifications::device_tokens};
use ogonek_notifications::NotificationType;
use ogonek_types::DeviceTokenPayload;
#[utoipa::path(
    post,
    path = "/register",
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
    device_tokens::upsert(&state.db, &claims.sub, &payload).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    post,
    path = "/request",
    responses(
        (status = 204, description = "Homework requested"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn request_hw(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<StatusCode, APIError> {
    let user = user::read_by_id(&state.db, &claims.sub).await?;

    state
        .notification_service
        .notify_teacher(
            &user.id,
            NotificationType::TeacherNotify {
                username: user.name,
            },
        )
        .await?;

    Ok(StatusCode::NO_CONTENT)
}
