use crate::api::error::APIError;
use crate::auth::error::AuthError;
use crate::auth::password::hash_password;
use crate::auth::{tokens, Claims};
use crate::db::crud::account::user;
use crate::schema::AppState;
use axum::extract::{Json, Query, State};
use hyper::StatusCode;

use crate::models::users::{InviteTokenParams, UserUpdate, UserWithInvite};

pub async fn fetch_user(
    State(state): State<AppState>,
    claims: Claims,
    Query(params): Query<InviteTokenParams>,
) -> Result<Json<UserWithInvite>, APIError> {
    let teacher = if let Some(invite) = params.invite {
        let teacher_id = tokens::decode_invite_token(invite).await?;

        Some(
            user::find_by_id(&state.db, &teacher_id)
                .await
                .map_err(|_| AuthError::InvalidToken)?,
        )
    } else {
        None
    };

    let user = user::find_by_id(&state.db, &claims.sub).await?;
    Ok(Json(UserWithInvite { user, teacher }))
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
