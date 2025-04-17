use crate::api::error::APIError;
use crate::auth::jwt::Claims;
use crate::models::profiles::{Profile, ProfileParams, ProfileUpdate};
use crate::models::ProfileWithTS;
use crate::schema::AppState;
use axum::extract::{Json, Query, State};

pub async fn upsert_profile(
    State(state): State<AppState>,
    claims: Claims,
    Json(payload): Json<ProfileUpdate>,
) -> Result<Json<Profile>, APIError> {
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
    Query(params): Query<ProfileParams>,
    claims: Claims,
) -> Result<Json<ProfileWithTS>, APIError> {
    let profile = sqlx::query_as!(
        Profile,
        r#"
        SELECT * FROM profile
        WHERE user_id = $1
        "#,
        claims.sub
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| APIError::NotFound("Profile not found".into()))?;

    let teacher_telegram_id = if params.is_student {
        sqlx::query!(
            r#"
                SELECT teacher_telegram_id FROM teacher_student
                WHERE student_id = $1
                "#,
            claims.sub
        )
        .fetch_optional(&state.db)
        .await?
        .map(|record| record.teacher_telegram_id)
        .flatten()
    } else {
        None
    };

    Ok(Json(ProfileWithTS {
        profile,
        teacher_telegram_id,
    }))
}
