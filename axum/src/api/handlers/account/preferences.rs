// src/api/account/preferences.rs
use crate::{
    api::error::APIError,
    auth::Claims,
    db::crud::core::account::preferences,
    schema::AppState,
    types::preferences::{UserPreferencesResponse, UserPreferencesUpdate},
};
use axum::{
    extract::{Json, State},
    http::StatusCode,
};

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
