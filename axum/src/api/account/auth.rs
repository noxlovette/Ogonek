use crate::api::error::APIError;
use crate::auth::Claims;
use crate::auth::error::AuthError;
use crate::auth::password::{hash_password, verify_password};
use crate::auth::tokens::{self, decode_token, generate_token};
use crate::db::crud::account::auth;
use crate::models::users::{AuthPayload, BindPayload, SignUpPayload, TokenPair};
use crate::models::{CreationId, InviteQuery, RefreshTokenPayload, RefreshTokenResponse};
use crate::schema::AppState;
use axum::extract::{Json, Query, State};
use hyper::StatusCode;
use validator::Validate;

pub async fn signup(
    State(state): State<AppState>,
    Json(payload): Json<SignUpPayload>,
) -> Result<Json<CreationId>, APIError> {
    if payload.username.is_empty() || payload.pass.is_empty() {
        return Err(APIError::InvalidCredentials);
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

pub async fn authorize(
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

pub async fn bind_student_to_teacher(
    State(state): State<AppState>,
    Json(payload): Json<BindPayload>,
) -> Result<StatusCode, AuthError> {
    let teacher_id = tokens::decode_invite_token(payload.invite_token).await?;

    auth::bind(&state.db, &teacher_id, &payload.student_id).await?;

    Ok(StatusCode::NO_CONTENT)
}
