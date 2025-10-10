use crate::{
    api::{AUTH_TAG, error::APIError},
    app::AppState,
    services::{
        AuthError, decode_token, generate_secure_token, generate_token, hash_password,
        verify_password,
    },
};

use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use ogonek_db::core::account::auth;
use ogonek_types::{
    AuthPayload, RefreshTokenPayload, RefreshTokenResponse, SignUpPayload, TokenPair, UserRole,
};
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
    State(mut state): State<AppState>,
    Json(payload): Json<SignUpPayload>,
) -> Result<(StatusCode, Json<String>), APIError> {
    if payload.username.is_empty() || payload.pass.is_empty() {
        return Err(APIError::AuthError(AuthError::InvalidCredentials));
    }
    let role = UserRole::from(payload.role.clone());
    if !role.can_self_assign() {
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
    let id = auth::signup(&state.db, &created).await?;

    let token = generate_secure_token();
    state
        .redis
        .set_verification_token(&created.email, &token, None)
        .await?;

    tokio::spawn(async move {
        if let Err(e) = state
            .ses
            .send_confirm_email(&created.email, &created.name, &created.role, &token)
            .await
        {
            tracing::error!("Error sending verification token: {e}")
        }
    });

    Ok((StatusCode::CREATED, Json(id)))
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
        return Err(APIError::AuthError(AuthError::InvalidCredentials));
    }
    payload.validate().map_err(|e| {
        eprintln!("{e:?}");
        APIError::AuthError(AuthError::InvalidCredentials)
    })?;

    let user = auth::authorise(&state.db, &payload).await?;

    if !verify_password(&user.pass, &payload.pass)? {
        return Err(APIError::AuthError(AuthError::AuthenticationFailed));
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
