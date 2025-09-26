use crate::{db::error::DbError, types::AuditLogCreate};
use nanoid::nanoid;

/// Insert a new audit log entry
pub async fn create(
    db: impl sqlx::Executor<'_, Database = sqlx::Postgres>,
    audit: &AuditLogCreate,
) -> Result<(), DbError> {
    let id = nanoid!();
    let empty_payload = serde_json::json!({});
    let empty_tags: Vec<String> = vec![];
    sqlx::query!(
        r#"
        INSERT INTO audit_logs (
            id, 
            event_type, 
            action, 
            outcome,
            user_id, 
            user_email, 
            user_role, 
            impersonated_by,
            resource_type, 
            resource_id, 
            resource_name,
            payload,
            session_id, 
            ip_address, 
            user_agent, 
            request_id,
            severity, 
            tags,
            occurred_at, 
            retention_until
        )
        VALUES (
            $1, 
            $2, 
            $3, 
            $4,
            $5, 
            $6, 
            $7, 
            $8,
            $9, 
            $10, 
            $11,
            $12,
            $13, 
            $14, 
            $15, 
            $16,
            $17, 
            $18,
            CURRENT_TIMESTAMP, 
            $19
        )
        "#,
        id,                                               // $1
        audit.event_type,                                 // $2
        audit.action,                                     // $3
        audit.outcome.as_deref().unwrap_or("success"),    // $4
        audit.user_id,                                    // $5
        audit.user_email,                                 // $6
        audit.user_role,                                  // $7
        audit.impersonated_by,                            // $8
        audit.resource_type,                              // $9
        audit.resource_id,                                // $10
        audit.resource_name,                              // $11
        audit.payload.as_ref().unwrap_or(&empty_payload), // $12
        audit.session_id,                                 // $13
        audit.ip_address,                                 // $14
        audit.user_agent,                                 // $15
        audit.request_id,                                 // $16
        audit.severity.as_deref().unwrap_or("info"),      // $17
        &audit.tags.as_ref().unwrap_or(&empty_tags),      // $18
        audit.retention_until                             // $19
    )
    .execute(db)
    .await?;

    Ok(())
}
