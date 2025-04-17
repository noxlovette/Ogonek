use crate::api::error::APIError;
use crate::auth::jwt::Claims;
use crate::models::profiles::{Profile, ProfileParams, ProfileUpdate, TeacherData};
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
            zoom_url,
            avatar_url,
            telegram_id
        )
        VALUES (
            $1,
            $2,
            $3,
            $4
        )
        ON CONFLICT (user_id)
        DO UPDATE SET
            zoom_url = COALESCE(EXCLUDED.zoom_url, profile.zoom_url),
            avatar_url = COALESCE(EXCLUDED.avatar_url, profile.avatar_url),
            telegram_id = COALESCE(EXCLUDED.telegram_id, profile.telegram_id)
        RETURNING *
        "#,
        claims.sub,
        payload.zoom_url,
        payload.avatar_url,
        payload.telegram_id,
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

    let teacher_data = if params.is_student == "true" {
        sqlx::query_as!(
            TeacherData,
            r#"
                SELECT
                    p.telegram_id as teacher_telegram_id,
                    p.zoom_url as teacher_zoom_url
                FROM teacher_student ts
                JOIN profile p ON ts.teacher_id = p.user_id
                WHERE ts.student_id = $1
                "#,
            claims.sub
        )
        .fetch_optional(&state.db)
        .await?
    } else {
        None
    };

    Ok(Json(ProfileWithTS {
        profile,
        teacher_data,
    }))
}
