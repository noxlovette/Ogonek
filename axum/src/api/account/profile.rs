use crate::api::USER_TAG;
use crate::api::error::APIError;
use crate::auth::Claims;
use crate::db::crud::core::account::profile;
use crate::schema::AppState;
use crate::types::Profile;
use crate::types::profiles::ProfileUpdate;
use axum::extract::{Json, State};
use axum::http::StatusCode;

/// Update the profile, or create a new one if already there
#[utoipa::path(
    patch,
    tag = USER_TAG,
    path = "/profile",
    request_body = ProfileUpdate,
    responses(
        (status = 204, description = "Profile updated successfully"),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn upsert_profile(
    State(state): State<AppState>,
    claims: Claims,
    Json(payload): Json<ProfileUpdate>,
) -> Result<StatusCode, APIError> {
    tracing::info!("{}", serde_json::to_string(&payload).unwrap_or_default());
    profile::upsert(&state.db, &claims.sub, &payload).await?;

    Ok(StatusCode::NO_CONTENT)
}

/// Fetch the profile
#[utoipa::path(
    get,
    tag = USER_TAG,
    path = "/profile",
    responses(
        (status = 200, description = "Profile details retrieved", body = Profile),
        (status = 401, description = "Unauthorized"),
    )
)]
pub async fn fetch_profile(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<Profile>, APIError> {
    let profile = profile::find_by_id(&state.db, &claims.sub).await?;

    Ok(Json(profile))
}
