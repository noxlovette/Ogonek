use crate::api::ADMIN_TAG;
use crate::api::error::APIError;
use crate::auth::Claims;
use crate::auth::password::hash_password;
use crate::db::crud::core::account::{auth, user};

use crate::db::crud::tracking;
use crate::schema::AppState;
use crate::tools::extractors::RequestMetadata;
use crate::types::{SignUpPayload, UserRole, AuditLogCreate}; // Add audit types
use axum::extract::{Json, State};
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
        return Err(APIError::InvalidCredentials);
    }

    let email = user::get_email(&state.db, &claims.sub).await?;
    
    let target_role = UserRole::from(payload.role.clone());
    if !target_role.can_be_assigned_by(&claims.role) {
        tracing::warn!(
            "User {} (role: {}) attempted to create user with role: {}",
            claims.sub,
            claims.role,
            target_role
        );
        
        // Audit failed authorization attempt
        let failed_audit = AuditLogCreate {
            event_type: "user.create".to_string(),
            action: "CREATE".to_string(),
            outcome: Some("failure".to_string()),
            user_id: Some(claims.sub.clone()),
            user_email: email, 
            user_role: claims.role.clone().to_string(),
            impersonated_by: None,
            resource_type: "user".to_string(),
            resource_id: None,
            resource_name: Some(payload.username.clone()),
            payload: Some(serde_json::json!({
                "attempted_role": payload.role,
                "reason": "insufficient_privileges",
                "target_username": payload.username,
                "target_email": payload.email
            })),
            session_id: metadata.session_id, 
            ip_address: metadata.ip_address, 
            user_agent: Some(metadata.user_agent),
            request_id: Some(metadata.request_id),
            severity: Some("warning".to_string()),
            tags: Some(vec!["authorization".to_string(), "security".to_string()]),
            retention_until: None,
        };
        
        // Log the failed attempt (don't fail the request if audit fails)
        if let Err(e) = tracking::create(&state.db, &failed_audit).await {
            tracing::error!("Failed to log audit event: {}", e);
        }
        
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
    
    // Create the user
    let id = auth::signup(&state.db, &created).await?;
    
    // Audit successful user creation
    let success_audit = AuditLogCreate {
        event_type: "user.create".to_string(),
        action: "CREATE".to_string(),
        outcome: Some("success".to_string()),
        user_id: Some(claims.sub.clone()),
        user_email: email, 
        user_role: claims.role.clone().to_string(),
        impersonated_by: None,
        resource_type: "user".to_string(),
        resource_id: Some(id.clone()),
        resource_name: Some(created.username.clone()),
        payload: Some(serde_json::json!({
            "created_user_id": id,
            "created_username": created.username,
            "created_email": created.email,
            "created_role": created.role,
            "created_name": created.name
        })),
        session_id: None,
        ip_address: None,
        user_agent: None,
        request_id: None,
        severity: Some("info".to_string()),
        tags: Some(vec!["user_management".to_string(), "account_creation".to_string()]),
        retention_until: None,
    };
    
    // Log successful creation (don't fail the request if audit fails)
    if let Err(e) = tracking::create(&state.db, &success_audit).await {
        tracing::error!("Failed to log audit event: {}", e);
    }
    
    Ok(Json(id))
}