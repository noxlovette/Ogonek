use axum::{
    Json,
    extract::{Query, State},
};
use ogonek_db::core::account::auth;
use ogonek_types::{BindPayload, InviteQuery};
use reqwest::StatusCode;

use crate::{
    AppError, AppState, Claims,
    api::AUTH_TAG,
    services::{AuthError, decode_invite_token, encode_invite_token},
};

/// Binds the student to the teacher
#[utoipa::path(
    post,
    path = "/bind",
    request_body = BindPayload,
    tag = AUTH_TAG,
    responses(
        (status = 204, description = "Student bound to teacher successfully"),
        (status = 400, description = "Invalid bind data"),
        (status = 401, description = "Invalid invite token")
    )
)]
pub async fn bind_student_to_teacher(
    State(state): State<AppState>,
    Json(payload): Json<BindPayload>,
) -> Result<StatusCode, AppError> {
    let teacher_id = decode_invite_token(payload.invite_token).await?;

    let _ = auth::bind(&state.db, &teacher_id, &payload.student_id)
        .await
        .map_err(|_| AuthError::InvalidToken);

    Ok(StatusCode::NO_CONTENT)
}

/// Generates the invite link for the teacher
#[utoipa::path(
    get,
    path = "/invite",
    params(
        ("isRegistered" = InviteQuery, Query, description = "Invite token")
    ),
    tag = AUTH_TAG,
    responses(
        (status = 200, description = "Invite link generated", body = String),
        (status = 401, description = "Unauthorized"),
        (status = 400, description = "Invalid invite token")
    )
)]
pub async fn generate_invite_link(
    claims: Claims,
    query: Query<InviteQuery>,
) -> Result<Json<String>, AppError> {
    let frontend_url = std::env::var("FRONTEND_URL")
        .unwrap_or_else(|_| "http://localhost:5173".to_string())
        .trim_end_matches('/')
        .to_string();

    let encoded = encode_invite_token(claims.sub).await?;

    tracing::info!("Encoded token: {encoded}");

    if query.is_registered == "true" {
        Ok(Json(format!("{frontend_url}/bind?invite={encoded}",)))
    } else {
        Ok(Json(format!("{frontend_url}/signup?invite={encoded}",)))
    }
}
