use crate::api::error::APIError;
use crate::auth::Claims;
use crate::db::crud::core::account::{profile, user};
use crate::db::crud::notifications::device_tokens::*;
use crate::notifications::dispatch_notification;
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
    upsert(&state.db, &claims.sub, &payload).await?;
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
    let teacher_telegram_id = profile::get_teacher_telegram_id(&state.db, &claims.sub).await?;
    let user = user::find_by_id(&state.db, &claims.sub).await?;
    let user_name = user.name;
    if let Some(telegram_id) = teacher_telegram_id {
        dispatch_notification(
            &state.bot_token,
            &state.http_client,
            &telegram_id,
            NotificationType::TeacherNotify {
                username: user_name,
            },
        )
        .await?;
    }
    Ok(StatusCode::NO_CONTENT)
}
