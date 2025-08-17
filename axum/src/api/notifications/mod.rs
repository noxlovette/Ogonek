use crate::api::error::APIError;
use crate::auth::Claims;
use crate::db::crud::core::account::user;
use crate::db::crud::notifications::device_tokens;
use crate::notifications::messages::NotificationType;
use crate::schema::AppState;
use crate::types::DeviceTokenPayload;
use axum::extract::{Json, State};
use axum::http::StatusCode;

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
    let user = user::find_by_id(&state.db, &claims.sub).await?;

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
