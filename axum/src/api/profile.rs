use crate::auth::jwt::Claims;
use crate::db::error::DbError;
use crate::db::init::AppState;
use crate::models::profiles::{Profile, ProfileUpdate};
use axum::extract::Json;
use axum::extract::State;

pub async fn upsert_profile(
    State(state): State<AppState>,
    claims: Claims,
    Json(payload): Json<ProfileUpdate>,
) -> Result<Json<Profile>, DbError> {
    let profile = sqlx::query_as!(
        Profile,
        r#"
        INSERT INTO profile (
            user_id,
            quizlet_url,
            zoom_url,
            bio,
            avatar_url,
            timezone
        )
        VALUES (
            $1, -- user_id
            $2, -- quizlet_url
            $3, -- zoom_url
            $4, -- bio
            $5, -- avatar_url
            $6  -- timezone
        )
        ON CONFLICT (user_id)
        DO UPDATE SET
            quizlet_url = COALESCE(EXCLUDED.quizlet_url, profile.quizlet_url),
            zoom_url = COALESCE(EXCLUDED.zoom_url, profile.zoom_url),
            bio = COALESCE(EXCLUDED.bio, profile.bio),
            avatar_url = COALESCE(EXCLUDED.avatar_url, profile.avatar_url),
            timezone = COALESCE(EXCLUDED.timezone, profile.timezone)
        RETURNING *
        "#,
        claims.sub, // $1
        payload.quizlet_url,
        payload.zoom_url,
        payload.bio,
        payload.avatar_url,
        payload.timezone,
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
