use crate::auth::error::AuthError;
use crate::auth::helpers::verify_password;
use crate::auth::helpers::{generate_refresh_token, generate_token, hash_password};
use crate::auth::jwt::Claims;
use crate::auth::jwt::RefreshClaims;
use crate::schema::AppState;
use crate::api::error::APIError;
use crate::models::users::{
    AuthBody, AuthPayload, BindPayload, InviteToken, SignUpBody, SignUpPayload, User,
};
use axum::extract::Json;
use axum::extract::State;
use axum::response::Response;
use hyper::StatusCode;
use nanoid::nanoid;
use validator::Validate;

use base64::{engine::general_purpose::URL_SAFE, Engine as _};

pub async fn signup(
    State(state): State<AppState>,
    Json(payload): Json<SignUpPayload>,
) -> Result<Json<SignUpBody>, APIError> {
    tracing::info!("Creating user");
    if payload.username.is_empty() || payload.pass.is_empty() {
        return Err(APIError::InvalidCredentials);
    }
    payload.validate().map_err(|e| {
        eprintln!("{:?}", e);
        APIError::InvalidCredentials
    })?;

    let SignUpPayload {
        name,
        username,
        email,
        role,
        pass,
    } = payload;

    let db = &state.db;
    let hashed_password = hash_password(&pass)?;
    let id = nanoid!();

    sqlx::query!(
        r#"
            INSERT INTO "user" (name, username, email, role, pass, verified, id)
            VALUES ($1, $2, $3, $4, $5, false, $6)
        "#,
        name,
        username,
        email,
        role,
        hashed_password,
        id
    )
    .execute(db)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(dbe) if dbe.constraint() == Some("user_username_key") => {
            APIError::AlreadyExists("Username already taken".into())
        }
        sqlx::Error::Database(dbe) if dbe.constraint() == Some("user_email_key") => {
            APIError::AlreadyExists("Email already registered".into())
        }
        _ => APIError::Database(e),
    })?;

    Ok(Json(SignUpBody { id }))
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

    let user = sqlx::query_as!(
        User,
        r#"
        SELECT username, email, role, id, name, pass, verified
        FROM "user" 
        WHERE username = $1
        "#,
        payload.username
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        eprintln!("{:?}", e);
        AuthError::WrongCredentials
    })?
    .ok_or_else(|| APIError::NotFound("User not found".into()))?;

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
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT username, email, role, id, name, pass, verified 
        FROM "user" 
        WHERE id = $1
        "#,
        claims.sub
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        eprintln!("{:?}", e);
        APIError::NotFound("User not Found".into())
    })?
    .ok_or(AuthError::UserNotFound)?;

    let token = generate_token(&user)?;
    Ok(AuthBody::into_refresh(token))
}

pub async fn generate_invite_link(claims: Claims) -> Result<Json<String>, AuthError> {
    if claims.role != "teacher" {
        return Err(AuthError::InvalidToken);
    }

    // Get frontend URL from env with a fallback
    let frontend_url = std::env::var("FRONTEND_URL")
        .unwrap_or_else(|_| "http://localhost".to_string())
        .trim_end_matches('/') // Remove trailing slash if present
        .to_string();

    let token = InviteToken::new(claims.sub);
    let encoded = URL_SAFE.encode(serde_json::to_string(&token).unwrap().as_bytes());

    Ok(Json(format!(
        "{}/auth/signup?invite={}",
        frontend_url, encoded
    )))
}

// New endpoint for binding students to teachers
pub async fn bind_student_to_teacher(
    State(state): State<AppState>,
    Json(payload): Json<BindPayload>,
) -> Result<StatusCode, AuthError> {
    let token: InviteToken = serde_json::from_str(
        &String::from_utf8(URL_SAFE.decode(&payload.invite_token).unwrap())
            .map_err(|_| AuthError::InvalidToken)?,
    )
    .map_err(|_| AuthError::InvalidToken)?;

    sqlx::query!(
        r#"
        INSERT INTO teacher_student (teacher_id, student_id)
        VALUES ($1, $2)
        ON CONFLICT DO NOTHING 
        "#,
        token.teacher_id,
        payload.student_id
    )
    .execute(&state.db)
    .await
    .map_err(|e| {
        eprintln!("Failed to bind student: {:?}", e);
        AuthError::InvalidCredentials
    })?;

    Ok(StatusCode::NO_CONTENT)
}
