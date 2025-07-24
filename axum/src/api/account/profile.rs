use crate::api::error::APIError;
use crate::auth::Claims;
use crate::db::crud::account::profile;
use crate::models::ProfileWithTS;
use crate::models::profiles::ProfileUpdate;
use crate::schema::AppState;
use axum::extract::{Json, State};
use axum::http::StatusCode;

/// Update the profile, or create a new one if already there
pub async fn upsert_profile(
    State(state): State<AppState>,
    claims: Claims,
    Json(payload): Json<ProfileUpdate>,
) -> Result<StatusCode, APIError> {
    tracing::info!("{}", serde_json::to_string(&payload).unwrap_or_default());
    profile::upsert(&state.db, &claims.sub, &payload).await?;

    Ok(StatusCode::NO_CONTENT)
}

/// Fetch the profile, WITHOUT teacher data even if that's a student requesing
pub async fn fetch_profile(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<ProfileWithTS>, APIError> {
    let profile_with_ts = profile::find_by_id(&state.db, &claims.sub, false).await?;

    Ok(Json(profile_with_ts))
}
