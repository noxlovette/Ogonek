use crate::db::error::DbError;
use crate::models::{
    CompositeStudent, DeckSmall, LessonSmall, Student, TaskSmall, UpdateStudentRequest,
};
use sqlx::PgPool;

pub async fn upsert(db: &PgPool, user_id: &str, student_id: &str) -> Result<(), DbError> {
    sqlx::query!(
        r#"
        INSERT INTO teacher_student (teacher_id, student_id)
        VALUES ($1, $2)
        ON CONFLICT (teacher_id, student_id) DO UPDATE SET status = 'active'
        "#,
        user_id,
        student_id
    )
    .execute(db)
    .await?;

    Ok(())
}

pub async fn find_all(db: &PgPool, user_id: &str) -> Result<Vec<Student>, DbError> {
    let students = sqlx::query_as!(
        Student,
        r#"
        SELECT u.username, u.email, u.id, u.name, ts.markdown, ts.student_telegram_id
        FROM "user" u
        INNER JOIN teacher_student ts ON u.id = ts.student_id
        WHERE ts.teacher_id = $1 AND ts.status = 'active'
        ORDER BY u.name ASC
        "#,
        user_id
    )
    .fetch_all(db)
    .await?;

    Ok(students)
}

pub async fn find_by_id_and_data(
    db: &PgPool,
    student_id: &str,
    user_id: &str,
) -> Result<CompositeStudent, DbError> {
    let mut tx = db.begin().await?;

    let student = sqlx::query_as!(
        Student,
        r#"
        SELECT u.username, u.email, u.id, u.name, ts.markdown, ts.student_telegram_id
        FROM "user" u
        INNER JOIN teacher_student ts ON u.id = ts.student_id
        WHERE ts.teacher_id = $1 AND student_id = $2
        "#,
        user_id,
        student_id,
    )
    .fetch_one(&mut *tx)
    .await?;

    let decks = sqlx::query_as!(
        DeckSmall,
        r#"
        SELECT id, name, description FROM decks
        WHERE (created_by = $1 AND assignee = $2)
        LIMIT 2
        "#,
        user_id,
        student.id
    )
    .fetch_all(&mut *tx)
    .await?;

    let lessons = sqlx::query_as!(
        LessonSmall,
        r#"
        SELECT id, title, topic, markdown, created_at
        FROM lessons
        WHERE (created_by = $1 AND assignee = $2)
        ORDER BY created_at desc
        LIMIT 2
        "#,
        user_id,
        student.id,
    )
    .fetch_all(&mut *tx)
    .await?;

    let tasks = sqlx::query_as!(
        TaskSmall,
        r#"
        SELECT id, title, markdown, completed, due_date
        FROM tasks
        WHERE (created_by = $1 AND assignee = $2 AND completed = false)
        LIMIT 2
        "#,
        user_id,
        student.id,
    )
    .fetch_all(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(CompositeStudent {
        student,
        decks,
        lessons,
        tasks,
    })
}

pub async fn delete(db: &PgPool, student_id: &str, user_id: &str) -> Result<(), DbError> {
    sqlx::query!(
        r#"
        UPDATE teacher_student
        SET status = 'inactive'
        WHERE teacher_id = $1 AND student_id = $2
        "#,
        user_id,
        student_id
    )
    .execute(db)
    .await?;

    Ok(())
}

pub async fn update(
    db: &PgPool,
    student_id: &str,
    user_id: &str,
    update: UpdateStudentRequest,
) -> Result<(), DbError> {
    sqlx::query!(
        r#"
        UPDATE teacher_student
        SET
            markdown = COALESCE($3, markdown),
            student_telegram_id = COALESCE($4, student_telegram_id)
        WHERE teacher_id = $1 AND student_id = $2
        "#,
        user_id,
        student_id,
        update.markdown,
        update.student_telegram_id
    )
    .execute(db)
    .await?;

    Ok(())
}
