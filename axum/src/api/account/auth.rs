use crate::api::AUTH_TAG;
use crate::api::error::APIError;
use crate::auth::Claims;
use crate::auth::error::AuthError;
use crate::auth::password::{hash_password, verify_password};
use crate::auth::tokens::{self, decode_token, generate_token};
use crate::db::crud::core::account::auth;
use crate::schema::AppState;
use crate::types::{AuthPayload, BindPayload, SignUpPayload, TokenPair, UserRole};
use crate::types::{InviteQuery, RefreshTokenPayload, RefreshTokenResponse};
use axum::extract::{Json, Query, State};
use axum::http::StatusCode;
use validator::Validate;
#[utoipa::path(
    post,
    path = "/signup",
    request_body = SignUpPayload,
    tag = AUTH_TAG, responses(
        (status = 201, description = "User registered successfully"),
        (status = 400, description = "Invalid registration data"),
        (status = 403, description = "Forbidden"),
        (status = 409, description = "User already exists")
    )
)]
pub async fn signup(
    State(state): State<AppState>,
    Json(payload): Json<SignUpPayload>,
) -> Result<Json<String>, APIError> {
    if payload.username.is_empty() || payload.pass.is_empty() {
        return Err(APIError::InvalidCredentials);
    }
    let role = UserRole::from(payload.role.clone());
    if !role.can_self_assign() {
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
#[utoipa::path(
    post,
    path = "/signin",
    request_body = AuthPayload,
    tag = AUTH_TAG,
    responses(
        (status = 200, description = "Authentication successful", body = TokenPair),
        (status = 401, description = "Invalid credentials")
    )
)]
pub async fn signin(
    State(state): State<AppState>,
    Json(payload): Json<AuthPayload>,
) -> Result<Json<TokenPair>, APIError> {
    if payload.username.is_empty() || payload.pass.is_empty() {
        return Err(APIError::InvalidCredentials);
    }
    payload.validate().map_err(|e| {
        eprintln!("{e:?}");
        APIError::InvalidCredentials
    })?;

    let user = auth::authorise(&state.db, &payload).await?;

    if !verify_password(&user.pass, &payload.pass)? {
        return Err(APIError::AuthenticationFailed);
    }

    let access_token = generate_token(&user.id, &user.role, 60 * 15)?;
    let refresh_token = generate_token(&user.id, &user.role, 60 * 60 * 24 * 30)?;

    Ok(Json(TokenPair::new(access_token, refresh_token)))
}

/// Receives the refresh token as json, gets it, then decodes, finds the user id, and generates a new access token
#[utoipa::path(
    post,
    path = "/refresh",
    request_body = RefreshTokenPayload,
    tag = AUTH_TAG,
    responses(
        (status = 200, description = "Token refreshed", body = RefreshTokenResponse),
        (status = 401, description = "Invalid refresh token")
    )
)]
pub async fn refresh(
    State(state): State<AppState>,
    Json(request): Json<RefreshTokenPayload>,
) -> Result<Json<RefreshTokenResponse>, APIError> {
    // Decode the refresh token to get user claims
    let refresh_claims = decode_token(&request.refresh_token)?;

    let user = auth::fetch_by_id(&state.db, &refresh_claims.sub).await?;
    let new_access_token = generate_token(&user.id, &user.role, 60 * 15)?;

    Ok(Json(RefreshTokenResponse {
        access_token: new_access_token,
    }))
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
) -> Result<Json<String>, AuthError> {
    let frontend_url = std::env::var("FRONTEND_URL")
        .unwrap_or_else(|_| "http://localhost:5173".to_string())
        .trim_end_matches('/')
        .to_string();

    let encoded = tokens::encode_invite_token(claims.sub).await?;

    tracing::info!("Encoded token: {encoded}");

    if query.is_registered == "true" {
        Ok(Json(format!("{frontend_url}/auth/bind?invite={encoded}",)))
    } else {
        Ok(Json(
            format!("{frontend_url}/auth/signup?invite={encoded}",),
        ))
    }
}

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
) -> Result<StatusCode, AuthError> {
    let teacher_id = tokens::decode_invite_token(payload.invite_token).await?;

    auth::bind(&state.db, &teacher_id, &payload.student_id).await?;

    Ok(StatusCode::NO_CONTENT)
}
