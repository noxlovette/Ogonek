use crate::api::ADMIN_TAG;
use crate::api::error::APIError;
use crate::auth::Claims;
use crate::auth::password::hash_password;
use crate::db::crud::core::account::auth;
use crate::schema::AppState;
use crate::types::SignUpPayload;
use crate::types::UserRole;
use axum::extract::{Json, State};
use validator::Validate;

#[utoipa::path(
    post,
    path = "/signup",
    request_body = SignUpPayload,
    tag = ADMIN_TAG, responses(
        (status = 201, description = "User registered successfully"),
        (status = 400, description = "Invalid registration data"),
        (status = 403, description = "Forbidden"),
        (status = 409, description = "User already exists")
    )
)]
pub async fn create_user(
    State(state): State<AppState>,
    claims: Claims,
    Json(payload): Json<SignUpPayload>,
) -> Result<Json<String>, APIError> {
    if payload.username.is_empty() || payload.pass.is_empty() {
        return Err(APIError::InvalidCredentials);
    }

    let target_role = UserRole::from(payload.role.clone());
    if !target_role.can_be_assigned_by(&claims.role) {
        tracing::warn!(
            "User {} (role: {}) attempted to create user with role: {}",
            claims.sub,
            claims.role,
            target_role
        );
        return Err(APIError::AccessDenied);
    }

    payload.validate().map_err(|e| {
        eprintln!("{e:?}");
        APIError::InvalidCredentials
    })?;

    let hashed_password = hash_password(&payload.pass)?;
    let created = SignUpPayload {
        pass: hashed_password,
        ..payload
    };
    let id = auth::signup(&state.db, &created).await?;

    Ok(Json(id))
}
