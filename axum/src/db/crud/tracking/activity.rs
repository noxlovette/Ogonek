use super::{ActionType, ModelType};
use crate::db::{crud::tracking::ActivityLog, error::DbError};
use sqlx::PgPool;

/// Logs activity in the activity
pub async fn log_activity(
    db: &PgPool,
    user_id: &str,
    model_id: &str,
    model_type: ModelType,
    action: ActionType,
    target_id: Option<&str>,
) -> Result<(), DbError> {
    sqlx::query!(
        r#"
        INSERT INTO activity_logs (user_id, model_type, model_id, action, target_user_id)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        user_id,             // who did it
        model_type.as_str(), // what type
        model_id,            // which entity
        action.as_str(),     // what action
        target_id            // who it affects
    )
    .execute(db)
    .await?;
    Ok(())
}

pub async fn get_activity(
    db: &PgPool,
    user_id: &str,
    model_type: ModelType,
) -> Result<Vec<ActivityLog>, DbError> {
    let activity = sqlx::query_as!(
        ActivityLog,
        r#"
        SELECT model_type, model_id, action, created_at FROM activity_logs
        WHERE (user_id = $1 OR target_user_id = $1) AND model_type = $2
        LIMIT 5
        "#,
        user_id,
        model_type.as_str(),
    )
    .fetch_all(db)
    .await?;

    Ok(activity)
}
