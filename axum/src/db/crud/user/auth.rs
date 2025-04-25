use crate::auth::error::AuthError;
use crate::db::error::DbError;
use crate::models::{AuthPayload, CreationId, SignUpPayload, User};
use nanoid::nanoid;
use sqlx::PgPool;

pub async fn signup(db: &PgPool, create: &SignUpPayload) -> Result<CreationId, DbError> {
    let id = nanoid!();

    let id = sqlx::query_as!(
        CreationId,
        r#"
            INSERT INTO "user" (name, username, email, role, pass, verified, id)
            VALUES ($1, $2, $3, $4, $5, false, $6)
            RETURNING id
        "#,
        create.name,
        create.username,
        create.email,
        create.role,
        create.pass,
        id
    )
    .fetch_one(db)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(dbe) if dbe.constraint() == Some("user_username_key") => {
            DbError::AlreadyExists("Username already taken".into())
        }
        sqlx::Error::Database(dbe) if dbe.constraint() == Some("user_email_key") => {
            DbError::AlreadyExists("Email already registered".into())
        }
        _ => DbError::Database(e),
    })?;

    Ok(id)
}

pub async fn authorise(db: &PgPool, user: &AuthPayload) -> Result<User, AuthError> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT username, email, role, id, name, pass, verified
        FROM "user"
        WHERE username = $1
        "#,
        user.username
    )
    .fetch_one(db)
    .await?;

    Ok(user)
}

pub async fn bind(db: &PgPool, teacher_id: &str, student_id: &str) -> Result<(), AuthError> {
    sqlx::query!(
        r#"
        INSERT INTO teacher_student (teacher_id, student_id)
        VALUES ($1, $2)
        ON CONFLICT DO NOTHING
        "#,
        teacher_id,
        student_id
    )
    .execute(db)
    .await?;

    Ok(())
}
pub async fn fetch_by_id(db: &PgPool, user_id: &str) -> Result<User, AuthError> {
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
