// src/api/account/preferences.rs
use crate::{api::error::APIError, app::AppState, services::Claims};
use axum::{
    extract::{Json, State},
    http::StatusCode,
};

use ogonek_db::core::account::preferences;

use ogonek_types::{UserPreferencesResponse, UserPreferencesUpdate};

pub async fn fetch_preferences(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<UserPreferencesResponse>, APIError> {
    let prefs = preferences::read_or_create_defaults(&state.db, &claims.sub).await?;

    let response = UserPreferencesResponse {
        auto_subscribe: prefs.auto_subscribe,
        email_notifications: prefs.email_notifications,
        push_notifications: prefs.push_notifications,
        theme: prefs.theme,
        language: prefs.language,
    };

    Ok(Json(response))
}

pub async fn update_preferences(
    State(state): State<AppState>,
    claims: Claims,
    Json(payload): Json<UserPreferencesUpdate>,
) -> Result<StatusCode, APIError> {
    preferences::upsert(&state.db, &claims.sub, &payload).await?;

    Ok(StatusCode::NO_CONTENT)
}
