use crate::auth::jwt::Claims;
use crate::db::error::DbError;
use crate::db::init::AppState;
use crate::models::profiles::{Profile, ProfileUpdate};
use axum::extract::Json;
use axum::extract::State;

pub async fn update_profile(
    State(state): State<AppState>,
    claims: Claims,
    Json(payload): Json<ProfileUpdate>,
) -> Result<Json<Profile>, DbError> {
    let profile = sqlx::query_as!(
        Profile,
        r#"
        UPDATE profile
        SET
            quizlet_url = COALESCE($1, quizlet_url),
            zoom_url = COALESCE($2, zoom_url),
            bio = COALESCE($3, bio),
            avatar_url = COALESCE($4, avatar_url),
            timezone = COALESCE($5, timezone)
        WHERE user_id = $6
        RETURNING *
        "#,
        payload.quizlet_url,
        payload.zoom_url,
        payload.bio,
        payload.avatar_url,
        payload.timezone,
        claims.sub,
    )
    .fetch_one(&state.db)
    .await?;

    Ok(Json(profile))
}

pub async fn fetch_profile(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<Option<Profile>>, DbError> {
    let profile = sqlx::query_as!(
        Profile,
        r#"
        SELECT * FROM profile
        WHERE user_id = $1
        "#,
        claims.sub
    )
    .fetch_optional(&state.db)
    .await?;
    Ok(Json(profile))
}
