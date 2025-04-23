use crate::api::error::APIError;
use crate::auth::claims::RefreshClaims;
use crate::auth::error::AuthError;
use crate::auth::password::{hash_password, verify_password};
use crate::auth::tokens::{generate_refresh_token, generate_token};
use crate::auth::Claims;
use crate::db::crud::user::auth;
use crate::models::users::{AuthBody, AuthPayload, BindPayload, InviteToken, SignUpPayload};
use crate::schema::AppState;
use axum::extract::Json;
use axum::extract::State;
use axum::response::Response;
use hyper::StatusCode;
use validator::Validate;

use base64::{engine::general_purpose::URL_SAFE, Engine as _};

pub async fn signup(
    State(state): State<AppState>,
    Json(payload): Json<SignUpPayload>,
) -> Result<StatusCode, APIError> {
    if payload.username.is_empty() || payload.pass.is_empty() {
        return Err(APIError::InvalidCredentials);
    }
    payload.validate().map_err(|e| {
        eprintln!("{:?}", e);
        APIError::InvalidCredentials
    })?;

    let hashed_password = hash_password(&payload.pass)?;
    let created = SignUpPayload {
        pass: hashed_password,
        ..payload
    };
    auth::signup(&state.db, &created).await?;

    Ok(StatusCode::CREATED)
}

pub async fn authorize(
    State(state): State<AppState>,
    Json(payload): Json<AuthPayload>,
) -> Result<Response, APIError> {
    if payload.username.is_empty() || payload.pass.is_empty() {
        return Err(APIError::InvalidCredentials);
    }
    payload.validate().map_err(|e| {
        eprintln!("{:?}", e);
        APIError::InvalidCredentials
    })?;

    let user = auth::authorise(&state.db, &payload).await?;

    if !verify_password(&user.pass, &payload.pass)? {
        return Err(APIError::AuthenticationFailed);
    }

    let token = generate_token(&user)?;
    let refresh_token = generate_refresh_token(&user)?;

    Ok(AuthBody::into_response(token, refresh_token))
}

pub async fn refresh(
    State(state): State<AppState>,
    claims: RefreshClaims,
) -> Result<Response, APIError> {
    let user = auth::fetch_by_id(&state.db, &claims.sub).await?;

    let token = generate_token(&user)?;
    Ok(AuthBody::into_refresh(token))
}

pub async fn generate_invite_link(claims: Claims) -> Result<Json<String>, AuthError> {
    if claims.role != "teacher" {
        return Err(AuthError::InvalidToken);
    }

    let frontend_url = std::env::var("FRONTEND_URL")
        .unwrap_or_else(|_| "http://localhost".to_string())
        .trim_end_matches('/')
        .to_string();

    let token = InviteToken::new(claims.sub);
    let encoded = URL_SAFE.encode(serde_json::to_string(&token).unwrap().as_bytes());

    Ok(Json(format!(
        "{}/auth/signup?invite={}",
        frontend_url, encoded
    )))
}

pub async fn bind_student_to_teacher(
    State(state): State<AppState>,
    Json(payload): Json<BindPayload>,
) -> Result<StatusCode, AuthError> {
    let token: InviteToken = serde_json::from_str(
        &String::from_utf8(URL_SAFE.decode(&payload.invite_token).unwrap())
            .map_err(|_| AuthError::InvalidToken)?,
    )
    .map_err(|_| AuthError::InvalidToken)?;

    auth::bind(&state.db, &token.teacher_id, &payload.student_id).await?;

    Ok(StatusCode::NO_CONTENT)
}
