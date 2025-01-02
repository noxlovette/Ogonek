use axum::extract::Json;
use axum::extract::Path;
use axum::extract::State;
use serde::{ Deserialize, Serialize };
use crate::auth::Token;
use crate::db::error::DbError;
use crate::db::AppState;

use crate::api::auth::SignUpPayload;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserBody {
    name: String,
    username: String,
    email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserUpdateBody {
    name: Option<String>,
    username: Option<String>,
    email: Option<String>,
    pass: Option<String>,
}

pub async fn fetch_user_self(
    State(state): State<AppState>,
    token: Token
) -> Result<Json<Option<UserBody>>, DbError> {
    tracing::info!("Attempting fetch for user");

    let db = &state.db;
    db.authenticate(token.as_str()).await?;

    let result: Vec<UserBody> = db
        .query("SELECT name, email, username FROM user WHERE id = $auth.id").await?
        .take(0)?;

    let user = result.into_iter().next();

    tracing::info!("User fetch successful");

    Ok(Json(user))
}

pub async fn fetch_user(
    State(state): State<AppState>,
    token: Token,
    id: Path<String>
) -> Result<Json<Option<UserBody>>, DbError> {
    tracing::info!("Attempting to fetch user");

    let db = &state.db;

    tracing::info!("token: {}", token.as_str());
    tracing::info!("id: {}", &*id);

    db.authenticate(token.as_str()).await?;

    let user = db.select(("user", &*id)).await?;

    dbg!(&user);

    tracing::info!("Fetch user successful");
    Ok(Json(user))
}

pub async fn update_user(
    State(state): State<AppState>,
    token: Token,
    id: Path<String>,
    Json(payload): Json<SignUpPayload>
) -> Result<Json<Option<UserBody>>, DbError> {
    tracing::info!("Attempting update for user");

    let db = &state.db;
    db.authenticate(token.as_str()).await?;

    let user = db.update(("user", &*id)).content(payload).await?;

    tracing::info!("User update successful");

    Ok(Json(user))
}

pub async fn delete_user(
    State(state): State<AppState>,
    token: Token,
    id: Path<String>
) -> Result<Json<Option<UserBody>>, DbError> {
    tracing::info!("Attempting user deletion");

    let db = &state.db;
    db.authenticate(token.as_str()).await?;

    let user = db.delete(("user", &*id)).await?;

    tracing::info!("Deleted");

    Ok(Json(user))
}

pub async fn list_users(
    State(state): State<AppState>,
    token: Token
) -> Result<Json<Vec<UserBody>>, DbError> {
    let db = &state.db;
    db.authenticate(token.as_str()).await?;

    let users = db.select("user").await?;

    Ok(Json(users))
}

// create user makes no sense because we have signup for this
