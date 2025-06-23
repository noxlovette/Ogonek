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
        SELECT id, title, priority, completed, due_date
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::UpdateStudentRequest;
    use crate::tests::create_test_user;
    use sqlx::PgPool;

    #[sqlx::test]
    async fn test_upsert_new_relationship(db: PgPool) {
        let teacher_id = create_test_user(&db, "teacher", "teacher@ogonek.app").await;
        let student_id = create_test_user(&db, "student", "student@ogonek.app").await;
        // Test upsert with new relationship
        let result = upsert(&db, &teacher_id, &student_id).await;
        assert!(result.is_ok());

        // Verify the relationship was created
        let count = sqlx::query!(
            "SELECT COUNT(*) as count FROM teacher_student WHERE teacher_id = $1 AND student_id = $2 AND status = 'active'",
            teacher_id,
            student_id
        )
        .fetch_one(&db)
        .await
        .unwrap();

        assert_eq!(count.count.unwrap(), 1);
    }

    #[sqlx::test]
    async fn test_upsert_existing_relationship(db: PgPool) {
        let teacher_id = create_test_user(&db, "teacher", "teacher@ogonek.app").await;
        let student_id = create_test_user(&db, "student", "student@ogonek.app").await;

        // Create initial relationship with inactive status
        sqlx::query!(
            "INSERT INTO teacher_student (teacher_id, student_id, status) VALUES ($1, $2, 'inactive')",
            teacher_id,
            student_id
        )
        .execute(&db)
        .await
        .unwrap();

        // Test upsert should reactivate the relationship
        let result = upsert(&db, &teacher_id, &student_id).await;
        assert!(result.is_ok());

        // Verify the relationship status was updated to active
        let status = sqlx::query!(
            "SELECT status FROM teacher_student WHERE teacher_id = $1 AND student_id = $2",
            teacher_id,
            student_id
        )
        .fetch_one(&db)
        .await
        .unwrap();

        assert_eq!(status.status.as_deref(), Some("active"));
    }

    #[sqlx::test]
    async fn test_find_all_active_students(db: PgPool) {
        let teacher_id = create_test_user(&db, "teacher", "teacher@ogonek.app").await;
        let student1_id = create_test_user(&db, "student1", "student1@ogonek.app").await;
        let student2_id = create_test_user(&db, "student2", "student2@ogonek.app").await;

        // Create active relationship for student1
        sqlx::query!(
            "INSERT INTO teacher_student (teacher_id, student_id, status, markdown, student_telegram_id) VALUES ($1, $2, 'active', 'test markdown', 'telegram123')",
            teacher_id,
            student1_id
        )
        .execute(&db)
        .await
        .unwrap();

        // Create inactive relationship for student2
        sqlx::query!(
            "INSERT INTO teacher_student (teacher_id, student_id, status) VALUES ($1, $2, 'inactive')",
            teacher_id,
            student2_id
        )
        .execute(&db)
        .await
        .unwrap();

        // Test find_all should only return active students
        let result = find_all(&db, &teacher_id).await;
        assert!(result.is_ok());

        let students = result.unwrap();
        assert_eq!(students.len(), 1);
        assert_eq!(students[0].id, student1_id);
        assert_eq!(students[0].name, "Test Name");
        assert_eq!(students[0].markdown.as_deref(), Some("test markdown"));
        assert_eq!(
            students[0].student_telegram_id.as_deref(),
            Some("telegram123")
        );
    }

    #[sqlx::test]
    async fn test_find_all_empty_result(db: PgPool) {
        let teacher_id = create_test_user(&db, "teacher", "teacher@ogonek.app").await;
        let result = find_all(&db, &teacher_id).await;
        assert!(result.is_ok());

        let students = result.unwrap();
        assert_eq!(students.len(), 0);
    }

    #[sqlx::test]
    async fn test_find_by_id_and_data_not_found(db: PgPool) {
        let teacher_id = create_test_user(&db, "teacher", "teacher@ogonek.app").await;
        let student_id = create_test_user(&db, "student", "student@ogonek.app").await;

        let result = find_by_id_and_data(&db, &student_id, &teacher_id).await;
        assert!(result.is_err());
    }

    #[sqlx::test]
    async fn test_delete_student_relationship(db: PgPool) {
        let teacher_id = create_test_user(&db, "teacher", "teacher@ogonek.app").await;
        let student_id = create_test_user(&db, "student", "student@ogonek.app").await;

        // Create relationship
        sqlx::query!(
            "INSERT INTO teacher_student (teacher_id, student_id, status) VALUES ($1, $2, 'active')",
            teacher_id,
            student_id
        )
        .execute(&db)
        .await
        .unwrap();

        // Test delete
        let result = delete(&db, &student_id, &teacher_id).await;
        assert!(result.is_ok());

        // Verify the relationship status was updated to inactive
        let status = sqlx::query!(
            "SELECT status FROM teacher_student WHERE teacher_id = $1 AND student_id = $2",
            teacher_id,
            student_id
        )
        .fetch_one(&db)
        .await
        .unwrap();

        assert_eq!(status.status.as_deref(), Some("inactive"));
    }

    #[sqlx::test]
    async fn test_delete_nonexistent_relationship(db: PgPool) {
        let teacher_id = create_test_user(&db, "teacher", "teacher@ogonek.app").await;
        let student_id = create_test_user(&db, "student", "student@ogonek.app").await;

        // Test delete on non-existent relationship should not error
        let result = delete(&db, &student_id, &teacher_id).await;
        assert!(result.is_ok());
    }

    #[sqlx::test]
    async fn test_update_student_partial(db: PgPool) {
        let teacher_id = create_test_user(&db, "teacher", "teacher@ogonek.app").await;
        let student_id = create_test_user(&db, "student", "student@ogonek.app").await;

        // create the relationship
        sqlx::query!(
            "INSERT INTO teacher_student (teacher_id, student_id, status, markdown, student_telegram_id) VALUES ($1, $2, 'active', 'test markdown', 'old-telegram')",
            teacher_id,
            student_id,
        )
        .execute(&db)
        .await
        .unwrap();

        // Test partial update (only markdown)
        let update_request = UpdateStudentRequest {
            markdown: Some("new markdown".to_string()),
            student_telegram_id: None,
        };

        let result = update(&db, &student_id, &teacher_id, update_request).await;
        assert!(result.is_ok());

        // Verify only markdown was updated
        let row = sqlx::query!(
            "SELECT markdown, student_telegram_id FROM teacher_student WHERE teacher_id = $1 AND student_id = $2",
            teacher_id,
            student_id
        )
        .fetch_one(&db)
        .await
        .unwrap();

        assert_eq!(row.markdown.as_deref(), Some("new markdown"));
        assert_eq!(row.student_telegram_id.as_deref(), Some("old-telegram"));
    }

    #[sqlx::test]
    async fn test_update_student_full(db: PgPool) {
        let teacher_id = create_test_user(&db, "teacher", "teacher@ogonek.app").await;
        let student_id = create_test_user(&db, "student", "student@ogonek.app").await;

        // create the relationship
        sqlx::query!(
        "INSERT INTO teacher_student (teacher_id, student_id, status, markdown, student_telegram_id) VALUES ($1, $2, 'active', 'test markdown', 'old-telegram')",
        teacher_id,
        student_id,
    )
    .execute(&db)
    .await
    .unwrap();

        // Test full update
        let update_request = UpdateStudentRequest {
            markdown: Some("new markdown".to_string()),
            student_telegram_id: Some("new_telegram".to_string()),
        };

        let result = update(&db, &student_id, &teacher_id, update_request).await;
        assert!(result.is_ok());

        // Verify both fields were updated
        let row = sqlx::query!(
            "SELECT markdown, student_telegram_id FROM teacher_student WHERE teacher_id = $1 AND student_id = $2",
            teacher_id,
            student_id
        )
        .fetch_one(&db)
        .await
        .unwrap();

        assert_eq!(row.markdown.as_deref(), Some("new markdown"));
        assert_eq!(row.student_telegram_id.as_deref(), Some("new_telegram"));
    }

    #[sqlx::test]
    async fn test_update_nonexistent_relationship(db: PgPool) {
        let teacher_id = create_test_user(&db, "teacher", "teacher@ogonek.app").await;
        let student_id = create_test_user(&db, "student", "student@ogonek.app").await;

        let update_request = UpdateStudentRequest {
            markdown: Some("new markdown".to_string()),
            student_telegram_id: Some("new_telegram".to_string()),
        };

        // Test update on non-existent relationship should not error
        let result = update(&db, &student_id, &teacher_id, update_request).await;
        assert!(result.is_ok());
    }
}
