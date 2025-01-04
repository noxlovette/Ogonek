use crate::auth::jwt::Token;
use crate::db::error::DbError;
use crate::db::init::AppState;
use axum::extract::Json;
use axum::extract::Path;
use axum::extract::State;

use crate::models::users::User;

pub async fn fetch_user(
    State(state): State<AppState>,
    token: Token,
    id: Path<String>,
) -> Result<Json<Option<User>>, DbError> {
    tracing::info!("Attempting to fetch user");

    let db = &state.db;
    db.authenticate(token.as_str()).await?;
    let user = db.select(("user", &*id)).await?;

    Ok(Json(user))
}

pub async fn update_user(
    State(state): State<AppState>,
    token: Token,
    id: Path<String>,
    Json(payload): Json<User>,
) -> Result<Json<Option<User>>, DbError> {
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
    id: Path<String>,
) -> Result<Json<Option<User>>, DbError> {
    tracing::info!("Attempting user deletion");

    let db = &state.db;
    db.authenticate(token.as_str()).await?;

    let user = db.delete(("user", &*id)).await?;

    tracing::info!("Deleted");

    Ok(Json(user))
}

pub async fn list_users(
    State(state): State<AppState>,
    token: Token,
) -> Result<Json<Vec<User>>, DbError> {
    let db = &state.db;
    db.authenticate(token.as_str()).await?;

    let users = db.select("user").await?;

    Ok(Json(users))
}

// create user makes no sense because we have signup for this
