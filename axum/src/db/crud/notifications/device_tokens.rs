use crate::db::error::DbError;
use crate::types::DeviceTokenPayload;
use sqlx::PgPool;

pub async fn upsert(
    db: &PgPool,
    user_id: &str,
    payload: &DeviceTokenPayload,
) -> Result<(), DbError> {
    sqlx::query!(
        r#"
        INSERT INTO device_tokens (id, user_id, token, platform)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (user_id, token) DO UPDATE SET
            platform = EXCLUDED.platform,
            created_at = NOW()
        "#,
        nanoid::nanoid!(),
        user_id,
        payload.token,
        payload.platform
    )
    .execute(db)
    .await?;

    Ok(())
}
