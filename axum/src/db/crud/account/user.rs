use crate::db::error::DbError;
use crate::models::{User, UserUpdate};
use sqlx::PgPool;

pub async fn find_by_id(db: &PgPool, user_id: &str) -> Result<User, DbError> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT username, email, role, id, name, pass, verified
        FROM "user"
        WHERE id = $1
        "#,
        user_id
    )
    .fetch_one(db)
    .await?;

    Ok(user)
}

pub async fn delete(db: &PgPool, user_id: &str) -> Result<(), DbError> {
    sqlx::query_as!(
        User,
        r#"
        DELETE FROM "user"
        WHERE id = $1
        "#,
        user_id
    )
    .execute(db)
    .await?;

    Ok(())
}

pub async fn update(db: &PgPool, user_id: &str, update: &UserUpdate) -> Result<(), DbError> {
    sqlx::query_as!(
        User,
        r#"
        UPDATE "user"
        SET
            name = COALESCE($1, name),
            username = COALESCE($2, username),
            email = COALESCE($3, email),
            pass = COALESCE($4, pass),
            role = COALESCE($5, role),
            verified = COALESCE($6, verified)
        WHERE id = $7
        "#,
        update.name,
        update.username,
        update.email,
        update.pass,
        update.role,
        update.verified,
        user_id
    )
    .execute(db)
    .await
    .map_err(|e| {
        if let sqlx::Error::Database(dbe) = &e {
            if let Some(constraint) = dbe.constraint() {
                if constraint == "user_username_key" {
                    return DbError::AlreadyExists("Username already taken".into());
                }
                if constraint == "user_email_key" {
                    return DbError::AlreadyExists("Email already taken".into());
                }
            }
        }
        tracing::error!("Database error when updating user: {:?}", e);
        DbError::Database(e)
    })?;

    Ok(())
}
