use crate::{
    api::{ADMIN_TAG, error::APIError},
    app::AppState,
    services::{AuditBuilder, AuthError, Claims, RequestMetadata, hash_password},
};
use axum::extract::{Json, State};
use ogonek_db::{
    core::account::{auth, user},
    tracking::audit,
};
use ogonek_types::{SignUpPayload, UserRole};
use validator::Validate;

#[utoipa::path(
    post,
    path = "/signup",
    request_body = SignUpPayload,
    tag = ADMIN_TAG,
    responses(
        (status = 201, description = "User registered successfully"),
        (status = 400, description = "Invalid registration data"),
        (status = 403, description = "Forbidden"),
        (status = 409, description = "User already exists")
    )
)]
pub async fn create_user(
    State(state): State<AppState>,
    claims: Claims,
    metadata: RequestMetadata,
    Json(payload): Json<SignUpPayload>,
) -> Result<Json<String>, APIError> {
    if payload.username.is_empty() || payload.pass.is_empty() {
        return Err(APIError::AuthError(AuthError::InvalidCredentials));
    }

    let email = user::read_email(&state.db, &claims.sub).await?;
    let tarread_role = UserRole::from(payload.role.clone());

    if !tarread_role.can_be_assigned_by(&claims.role) {
        tracing::warn!(
            "User {} (role: {}) attempted to create user with role: {}",
            claims.sub,
            claims.role,
            tarread_role
        );

        let failed_audit = AuditBuilder::user_operation("CREATE", &claims, email)
            .failed()
            .security_event()
            .resource_name(payload.username.clone())
            .with_metadata(&metadata)
            .payload(serde_json::json!({
                "attempted_role": payload.role,
                "reason": "insufficient_privileges",
                "tarread_username": payload.username,
                "tarread_email": payload.email
            }))
            .tag("authorization")
            .build();

        audit::create(&state.db, &failed_audit).await?;
        return Err(APIError::AuthError(AuthError::AccessDenied));
    }

    payload.validate().map_err(|e| {
        eprintln!("{e:?}");
        APIError::AuthError(AuthError::InvalidCredentials)
    })?;

    let hashed_password = hash_password(&payload.pass)?;
    let created = SignUpPayload {
        pass: hashed_password,
        ..payload
    };

    let user_id = auth::signup(&state.db, &created).await?;

    let success_audit = AuditBuilder::user_operation("CREATE", &claims, email)
        .resource_id(user_id.clone())
        .resource_name(created.username.clone())
        .payload(serde_json::json!({
            "created_user_id": user_id,
            "created_username": created.username,
            "created_email": created.email,
            "created_role": created.role,
            "created_name": created.name
        }))
        .tag("account_creation")
        .build();

    audit::create(&state.db, &success_audit).await?;

    Ok(Json(user_id))
}
