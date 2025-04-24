use crate::api::error::APIError;
use crate::auth::claims::RefreshClaims;
use crate::auth::error::AuthError;
use crate::auth::password::{hash_password, verify_password};
use crate::auth::tokens::{self, generate_refresh_token, generate_token};
use crate::auth::Claims;
use crate::db::crud::user::auth;
use crate::models::users::{AuthBody, AuthPayload, BindParams, BindPayload, SignUpPayload};
use crate::schema::AppState;
use axum::extract::{Json, Query, State};
use axum::response::Response;
use hyper::StatusCode;
use validator::Validate;

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

pub async fn generate_invite_link(
    claims: Claims,
    Query(params): Query<BindParams>,
) -> Result<Json<String>, AuthError> {
    if claims.role != "teacher" {
        return Err(AuthError::InvalidToken);
    }

    let frontend_url = std::env::var("FRONTEND_URL")
        .unwrap_or_else(|_| "http://localhost:5173".to_string())
        .trim_end_matches('/')
        .to_string();

    let encoded = tokens::encode_invite_token(claims.sub).await?;

    if params.is_registered == "true" {
        Ok(Json(format!(
            "{}/auth/bind?invite={}",
            frontend_url, encoded
        )))
    } else {
        Ok(Json(format!(
            "{}/auth/signup?invite={}",
            frontend_url, encoded
        )))
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
