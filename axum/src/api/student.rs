use crate::auth::jwt::Claims;
use crate::db::error::DbError;
use crate::db::init::AppState;
use crate::models::students::AddStudentRequest;
use crate::models::users::User;
use axum::extract::Json;
use axum::extract::State;
use axum::http::StatusCode;

// does not handle the case of 'resurrection'
pub async fn upsert_student(
    claims: Claims, // from auth middleware
    State(state): State<AppState>,
    Json(payload): Json<AddStudentRequest>,
) -> Result<StatusCode, DbError> {
    // Insert the relationship

    if claims.role != "teacher" {
        return Ok(StatusCode::UNAUTHORIZED);
    }
    sqlx::query!(
        r#"
        INSERT INTO teacher_student (teacher_id, student_id)
        VALUES ($1, $2)
        ON CONFLICT (teacher_id, student_id) DO UPDATE SET status = 'active'
        "#,
        claims.sub, // teacher's ID from auth
        payload.student_id
    )
    .execute(&state.db)
    .await
    .map_err(|e| {
        eprintln!("Failed to upsert student: {:?}", e);
        // You might want to handle unique constraint violations separately
        if e.to_string().contains("unique constraint") {
            return DbError::AlreadyExists;
        }
        DbError::Db
    })?;

    Ok(StatusCode::CREATED)
}

pub async fn remove_student(
    claims: Claims,
    State(state): State<AppState>,
    Json(payload): Json<AddStudentRequest>,
) -> Result<StatusCode, DbError> {
    // Soft delete the relationship
    sqlx::query!(
        r#"
        UPDATE teacher_student
        SET status = 'inactive'
        WHERE teacher_id = $1 AND student_id = $2
        "#,
        claims.sub,
        payload.student_id
    )
    .execute(&state.db)
    .await
    .map_err(|e| {
        eprintln!("Failed to remove student: {:?}", e);
        DbError::Db
    })?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn list_teacher_students(
    claims: Claims,
    State(state): State<AppState>,
) -> Result<Json<Vec<User>>, DbError> {
    let users = sqlx::query_as!(
        User,
        r#"
        SELECT u.username, u.email, u.role, u.id, u.name, u.pass, u.verified
        FROM "user" u
        INNER JOIN teacher_student ts ON u.id = ts.student_id
        WHERE ts.teacher_id = $1 AND ts.status = 'active'
        "#,
        claims.sub
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        eprintln!("{:?}", e);
        DbError::Db
    })?;

    Ok(Json(users))
}
