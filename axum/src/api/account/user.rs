use crate::api::error::APIError;
use crate::auth::password::hash_password;
use crate::auth::{Claims, tokens};
use crate::db::crud::account::user;
use crate::models::User;
use crate::schema::AppState;
use axum::extract::{Json, Query, State};
use hyper::StatusCode;

use crate::models::users::UserUpdate;

pub async fn fetch_inviter(
    State(state): State<AppState>,
    Query(invite): Query<String>,
) -> Result<Json<User>, APIError> {
    let teacher_id = tokens::decode_invite_token(invite).await?;
    let inviter = user::find_by_id(&state.db, &teacher_id).await?;

    Ok(Json(inviter))
}
pub async fn fetch_me(
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
