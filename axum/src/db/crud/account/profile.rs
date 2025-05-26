use crate::db::error::DbError;
use crate::models::{Profile, ProfileParams, ProfileUpdate, ProfileWithTS, TeacherData};
use sqlx::PgPool;

pub async fn find_by_id(
    db: &PgPool,
    user_id: &str,
    params: &ProfileParams,
) -> Result<ProfileWithTS, DbError> {
    let profile = sqlx::query_as!(
        Profile,
        r#"
        SELECT * FROM profile
        WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_one(db)
    .await?;

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
            user_id
        )
        .fetch_optional(db)
        .await?
    } else {
        None
    };

    Ok(ProfileWithTS {
        profile,
        teacher_data,
    })
}

pub async fn upsert(db: &PgPool, user_id: &str, update: &ProfileUpdate) -> Result<(), DbError> {
    sqlx::query_as!(
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
        "#,
        user_id,
        update.zoom_url,
        update.avatar_url,
        update.telegram_id,
    )
    .execute(db)
    .await?;

    Ok(())
}
