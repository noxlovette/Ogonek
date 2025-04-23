use crate::api::error::APIError;
use crate::auth::password::hash_password;
use crate::auth::Claims;
use crate::db::crud::user::user;
use crate::schema::AppState;
use axum::extract::Json;
use axum::extract::State;
use hyper::StatusCode;

use crate::models::users::{User, UserUpdate};

pub async fn fetch_user(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<User>, APIError> {
    let user = user::find_by_id(&state.db, &claims.sub).await?;
    Ok(Json(user))
}

pub async fn delete_user(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<StatusCode, APIError> {
    user::delete(&state.db, &claims.sub).await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn update_user(
    State(state): State<AppState>,
    claims: Claims,
    Json(payload): Json<UserUpdate>,
) -> Result<StatusCode, APIError> {
    let hashed_pass = match payload.pass {
        Some(ref pass) => Some(hash_password(pass).map_err(|_| APIError::PasswordHash)?),
        None => None,
    };

    let update = UserUpdate {
        pass: hashed_pass,
        ..payload
    };

    user::update(&state.db, &claims.sub, &update).await?;

    Ok(StatusCode::NO_CONTENT)
}
