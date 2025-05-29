use crate::api::error::APIError;
use crate::auth::Claims;
use crate::db::crud::account::profile;
use crate::models::profiles::{ProfileParams, ProfileUpdate};
use crate::models::ProfileWithTS;
use crate::schema::AppState;
use axum::extract::{Json, Query, State};
use hyper::StatusCode;

pub async fn upsert_profile(
    State(state): State<AppState>,
    claims: Claims,
    Json(payload): Json<ProfileUpdate>,
) -> Result<StatusCode, APIError> {
    tracing::info!("{}", serde_json::to_string(&payload).unwrap_or_default());
    profile::upsert(&state.db, &claims.sub, &payload).await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn fetch_profile(
    State(state): State<AppState>,
    Query(params): Query<ProfileParams>,
    claims: Claims,
) -> Result<Json<ProfileWithTS>, APIError> {
    let profile_with_ts = profile::find_by_id(&state.db, &claims.sub, &params).await?;

    Ok(Json(profile_with_ts))
}
