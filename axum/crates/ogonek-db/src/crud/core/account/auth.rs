use crate::{
    auth::error::AuthError,
    db::error::DbError,
    types::{AuthPayload, SignUpPayload, User},
};
use nanoid::nanoid;
use sqlx::PgPool;

pub async fn signup(db: &PgPool, create: &SignUpPayload) -> Result<String, DbError> {
    let id = nanoid!();

    let id = sqlx::query_scalar!(
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
        SELECT username, email, role, id, name, pass
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
        SELECT username, email, role, id, name, pass
        FROM "user"
        WHERE id = $1
        "#,
        user_id
    )
    .fetch_one(db)
    .await?;

    Ok(user)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{AuthPayload, SignUpPayload, UserRole};
    use sqlx::PgPool;

    #[sqlx::test]
    async fn test_signup_success(pool: PgPool) -> sqlx::Result<()> {
        let signup_data = SignUpPayload {
            name: "John Doe".to_string(),
            username: "johndoe".to_string(),
            email: "john@example.com".to_string(),
            role: "student".to_string(),
            pass: "hashedpassword123".to_string(),
        };

        let result = signup(&pool, &signup_data).await;
        assert!(result.is_ok());

        let creation_id = result.unwrap();
        assert!(!creation_id.is_empty());

        // Verify user was created in database
        let user = sqlx::query!(
            r#"SELECT name, username, email, role, verified FROM "user" WHERE id = $1"#,
            creation_id
        )
        .fetch_one(&pool)
        .await?;

        assert_eq!(user.name, "John Doe");
        assert_eq!(user.username, "johndoe");
        assert_eq!(user.email, "john@example.com");
        assert_eq!(user.role, "student");
        assert!(!user.verified);

        Ok(())
    }

    #[sqlx::test]
    async fn test_signup_duplicate_username(pool: PgPool) -> sqlx::Result<()> {
        let signup_data = SignUpPayload {
            name: "John Doe".to_string(),
            username: "johndoe".to_string(),
            email: "john@example.com".to_string(),
            role: "student".to_string(),
            pass: "hashedpassword123".to_string(),
        };

        // First signup should succeed
        let result1 = signup(&pool, &signup_data).await;
        assert!(result1.is_ok());

        // Second signup with same username but different email should fail
        let signup_data2 = SignUpPayload {
            email: "john2@example.com".to_string(),
            ..signup_data
        };

        let result2 = signup(&pool, &signup_data2).await;
        assert!(result2.is_err());

        Ok(())
    }

    #[sqlx::test]
    async fn test_signup_duplicate_email(pool: PgPool) -> sqlx::Result<()> {
        let signup_data = SignUpPayload {
            name: "John Doe".to_string(),
            username: "johndoe".to_string(),
            email: "john@example.com".to_string(),
            role: "student".to_string(),
            pass: "hashedpassword123".to_string(),
        };

        // First signup should succeed
        let result1 = signup(&pool, &signup_data).await;
        assert!(result1.is_ok());

        // Second signup with same email but different username should fail
        let signup_data2 = SignUpPayload {
            username: "johndoe2".to_string(),
            ..signup_data
        };

        let result2 = signup(&pool, &signup_data2).await;
        assert!(result2.is_err());

        Ok(())
    }

    #[sqlx::test]
    async fn test_authorise_success(pool: PgPool) -> sqlx::Result<()> {
        // First create a user
        let signup_data = SignUpPayload {
            name: "Jane Doe".to_string(),
            username: "janedoe".to_string(),
            email: "jane@example.com".to_string(),
            role: "teacher".to_string(),
            pass: "hashedpassword456".to_string(),
        };

        let creation_result = signup(&pool, &signup_data).await.unwrap();

        // Now test authorisation
        let auth_payload = AuthPayload {
            username: "janedoe".to_string(),
            pass: "plainpassword456".to_string(), /* Note: this would normally be checked against the hash */
        };

        let result = authorise(&pool, &auth_payload).await;
        assert!(result.is_ok());

        let user = result.unwrap();
        assert_eq!(user.username, "janedoe");
        assert_eq!(user.email, "jane@example.com");
        assert_eq!(user.role, UserRole::Teacher);
        assert_eq!(user.name, "Jane Doe");
        assert_eq!(user.id, creation_result);

        Ok(())
    }

    #[sqlx::test]
    async fn test_authorise_user_not_found(pool: PgPool) -> sqlx::Result<()> {
        let auth_payload = AuthPayload {
            username: "nonexistent".to_string(),
            pass: "password".to_string(),
        };

        let result = authorise(&pool, &auth_payload).await;
        assert!(result.is_err());

        Ok(())
    }

    #[sqlx::test]
    async fn test_bind_success(pool: PgPool) -> sqlx::Result<()> {
        // Create teacher
        let teacher_signup = SignUpPayload {
            name: "Teacher Smith".to_string(),
            username: "teacher".to_string(),
            email: "teacher@example.com".to_string(),
            role: "teacher".to_string(),
            pass: "hashedpass".to_string(),
        };
        let teacher_id = signup(&pool, &teacher_signup).await.unwrap();

        // Create student
        let student_signup = SignUpPayload {
            name: "Student Jones".to_string(),
            username: "student".to_string(),
            email: "student@example.com".to_string(),
            role: "student".to_string(),
            pass: "hashedpass".to_string(),
        };
        let student_id = signup(&pool, &student_signup).await.unwrap();

        // Test binding
        let result = bind(&pool, &teacher_id, &student_id).await;
        assert!(result.is_ok());

        // Verify binding exists
        let binding = sqlx::query!(
            "SELECT teacher_id, student_id FROM teacher_student WHERE teacher_id = $1 AND student_id = $2",
            teacher_id,
            student_id
        )
        .fetch_one(&pool)
        .await?;

        assert_eq!(binding.teacher_id, teacher_id);
        assert_eq!(binding.student_id, student_id);

        Ok(())
    }

    #[sqlx::test]
    async fn test_bind_duplicate_ignored(pool: PgPool) -> sqlx::Result<()> {
        // Create teacher and student
        let teacher_signup = SignUpPayload {
            name: "Teacher Smith".to_string(),
            username: "teacher2".to_string(),
            email: "teacher2@example.com".to_string(),
            role: "teacher".to_string(),
            pass: "hashedpass".to_string(),
        };
        let teacher_id = signup(&pool, &teacher_signup).await.unwrap();

        let student_signup = SignUpPayload {
            name: "Student Jones".to_string(),
            username: "student2".to_string(),
            email: "student2@example.com".to_string(),
            role: "student".to_string(),
            pass: "hashedpass".to_string(),
        };
        let student_id = signup(&pool, &student_signup).await.unwrap();

        // First binding should succeed
        let result1 = bind(&pool, &teacher_id, &student_id).await;
        assert!(result1.is_ok());

        // Second binding should also succeed (conflict ignored)
        let result2 = bind(&pool, &teacher_id, &student_id).await;
        assert!(result2.is_ok());

        // Verify only one binding exists
        let count = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM teacher_student WHERE teacher_id = $1 AND student_id = $2",
            teacher_id,
            student_id
        )
        .fetch_one(&pool)
        .await?;

        assert_eq!(count, Some(1));

        Ok(())
    }

    #[sqlx::test]
    async fn test_fetch_by_id_success(pool: PgPool) -> sqlx::Result<()> {
        // Create a user
        let signup_data = SignUpPayload {
            name: "Alice Cooper".to_string(),
            username: "alice".to_string(),
            email: "alice@example.com".to_string(),
            role: "admin".to_string(),
            pass: "hashedpass789".to_string(),
        };

        let creation_result = signup(&pool, &signup_data).await.unwrap();

        // Test fetch by ID
        let result = fetch_by_id(&pool, &creation_result).await;
        assert!(result.is_ok());

        let user = result.unwrap();
        assert_eq!(user.id, creation_result);
        assert_eq!(user.username, "alice");
        assert_eq!(user.email, "alice@example.com");
        assert_eq!(user.role, UserRole::Admin);
        assert_eq!(user.name, "Alice Cooper");

        Ok(())
    }

    #[sqlx::test]
    async fn test_fetch_by_id_not_found(pool: PgPool) -> sqlx::Result<()> {
        let result = fetch_by_id(&pool, "nonexistent-id").await;
        assert!(result.is_err());

        Ok(())
    }

    #[sqlx::test]
    async fn test_bind_nonexistent_users(pool: PgPool) -> sqlx::Result<()> {
        let result = bind(&pool, "fake-teacher-id", "fake-student-id").await;
        assert!(result.is_err());

        Ok(())
    }
}
