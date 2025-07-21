// src/api/account/preferences.rs
use crate::api::error::APIError;
use crate::auth::Claims;
use crate::db::crud::account::preferences;
use crate::models::preferences::{UserPreferencesResponse, UserPreferencesUpdate};
use crate::schema::AppState;
use axum::extract::{Json, State};
use hyper::StatusCode;

pub async fn get_preferences(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<UserPreferencesResponse>, APIError> {
    let prefs = preferences::get_or_create_defaults(&state.db, &claims.sub).await?;

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

// Convenience endpoint to just get the auto_subscribe setting
pub async fn get_auto_subscribe(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<bool>, APIError> {
    let prefs = preferences::get_or_create_defaults(&state.db, &claims.sub).await?;
    Ok(Json(prefs.auto_subscribe))
}
