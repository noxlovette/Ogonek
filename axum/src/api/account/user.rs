use crate::api::USER_TAG;
use crate::api::error::APIError;
use crate::auth::password::hash_password;
use crate::auth::{Claims, tokens};
use crate::db::crud::account::user;
use crate::models::{InviterQuery, User};
use crate::schema::AppState;
use axum::extract::{Json, Query, State};
use axum::http::StatusCode;

use crate::models::users::UserUpdate;
/// Gets the inviter's credentials
#[utoipa::path(
    get,
    tag=USER_TAG,
    path = "/inviter",
    params(
        ("invite" = Option<String>, Query, description = "Invite token")
    ),
    responses(
        (status = 200, description = "Inviter details retrieved", body = User),
        (status = 401, description = "Unauthorized"),
        (status = 400, description = "Invalid invite token")
    ),
    security(("api_key" = []))
)]

pub async fn fetch_inviter(
    State(state): State<AppState>,
    query: Query<InviterQuery>,
) -> Result<Json<User>, APIError> {
    let teacher_id = tokens::decode_invite_token(query.invite.clone()).await?;
    let inviter = user::find_by_id(&state.db, &teacher_id).await?;

    Ok(Json(inviter))
}

/// Fetches self for the user
#[utoipa::path(
    get,
    tag=USER_TAG,
    path = "/me",
    responses(
        (status = 200, description = "User details retrieved", body = User),
        (status = 401, description = "Unauthorized"),
    ),
    security(("api_key" = []))
)]
pub async fn fetch_me(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<User>, APIError> {
    let user = user::find_by_id(&state.db, &claims.sub).await?;

    Ok(Json(user))
}

/// Deletes user by their claims
#[utoipa::path(
    delete,
    tag=USER_TAG,
    path = "/user",
    responses(
        (status = 204, description = "User deleted successfully"),
        (status = 401, description = "Unauthorized")
    ),
    security(("api_key" = []))
)]
pub async fn delete_user(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<StatusCode, APIError> {
    user::delete(&state.db, &claims.sub).await?;

    Ok(StatusCode::NO_CONTENT)
}
/// Updates the user by their claims
#[utoipa::path(
    patch,
    tag=USER_TAG,
    path = "/user",
    request_body = UserUpdate,
    responses(
        (status = 204, description = "User updated successfully"),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized")
    ),
    security(("api_key" = []))
)]
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
