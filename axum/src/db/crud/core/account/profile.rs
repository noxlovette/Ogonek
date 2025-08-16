use crate::db::error::DbError;
use crate::types::{Profile, ProfileUpdate};
use sqlx::PgPool;

pub async fn find_by_id(db: &PgPool, user_id: &str) -> Result<Profile, DbError> {
    let profile = sqlx::query_as!(
        Profile,
        r#"
        SELECT video_call_url, avatar_url, telegram_id, user_id FROM profile
        WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_one(db)
    .await?;

    Ok(profile)
}

pub async fn get_call_url(
    db: impl sqlx::Executor<'_, Database = sqlx::Postgres>,
    user_id: &str,
) -> Result<Option<String>, DbError> {
    let td = sqlx::query_scalar!(
        r#"
        SELECT
            p.video_call_url
        FROM teacher_student ts
        JOIN profile p ON ts.teacher_id = p.user_id
        WHERE ts.student_id = $1
        "#,
        user_id,
    )
    .fetch_optional(db)
    .await?
    .flatten();

    Ok(td)
}

pub async fn get_teacher_telegram_id(
    db: impl sqlx::Executor<'_, Database = sqlx::Postgres>,
    user_id: &str,
) -> Result<Option<String>, DbError> {
    let td = sqlx::query_scalar!(
        r#"
        SELECT
            p.telegram_id
        FROM teacher_student ts
        JOIN profile p ON ts.teacher_id = p.user_id
        WHERE ts.student_id = $1
        "#,
        user_id,
    )
    .fetch_optional(db)
    .await?
    .flatten();

    Ok(td)
}
pub async fn upsert(db: &PgPool, user_id: &str, update: &ProfileUpdate) -> Result<(), DbError> {
    sqlx::query_as!(
        Profile,
        r#"
        INSERT INTO profile (
            user_id,
            video_call_url,
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
            video_call_url = COALESCE(EXCLUDED.video_call_url, profile.video_call_url),
            avatar_url = COALESCE(EXCLUDED.avatar_url, profile.avatar_url),
            telegram_id = COALESCE(EXCLUDED.telegram_id, profile.telegram_id)
        "#,
        user_id,
        update.video_call_url,
        update.avatar_url,
        update.telegram_id,
    )
    .execute(db)
    .await?;

    Ok(())
}

pub async fn get_telegram_id(db: &PgPool, user_id: &str) -> Result<Option<String>, DbError> {
    let telegram_id = sqlx::query_scalar!(
        r#"
        SELECT telegram_id FROM profile
        WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_one(db)
    .await?;

    Ok(telegram_id)
}
