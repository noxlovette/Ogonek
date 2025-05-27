use crate::db::error::DbError;
use crate::models::{Profile, ProfileParams, ProfileUpdate, ProfileWithTS, TeacherData};
use sqlx::PgPool;

pub async fn find_by_id(
    db: &PgPool,
    user_id: &str,
    params: &ProfileParams,
) -> Result<ProfileWithTS, DbError> {
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

    let teacher_data = if params.is_student == "true" {
        sqlx::query_as!(
            TeacherData,
            r#"
                SELECT
                    p.telegram_id as teacher_telegram_id,
                    p.video_call_url as teacher_video_call_url
                FROM teacher_student ts
                JOIN profile p ON ts.teacher_id = p.user_id
                WHERE ts.student_id = $1
                "#,
            user_id
        )
        .fetch_optional(db)
        .await?
    } else {
        None
    };

    Ok(ProfileWithTS {
        profile,
        teacher_data,
    })
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Profile, ProfileParams, ProfileUpdate};
    use crate::tests::create_test_user;
    use sqlx::PgPool;

    #[sqlx::test]
    async fn test_find_by_id_profile_exists(db: PgPool) {
        // Setup test data
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;

        // Test params for non-student
        let params = ProfileParams {
            is_student: "false".to_string(),
        };

        // Call function
        let result = find_by_id(&db, &user_id, &params).await;

        // Assertions
        assert!(result.is_ok());
        let profile_with_ts = result.unwrap();
        assert_eq!(profile_with_ts.profile.user_id, user_id);
    }

    #[sqlx::test]
    async fn test_find_by_id_profile_not_found(db: PgPool) {
        let user_id = nanoid::nanoid!();
        let params = ProfileParams {
            is_student: "false".to_string(),
        };

        let result = find_by_id(&db, &user_id, &params).await;

        assert!(result.is_err());
    }

    #[sqlx::test]
    async fn test_find_by_id_student_with_teacher(db: PgPool) {
        // Setup teacher profile
        let teacher_id = create_test_user(&db, "teacher", "teacher@ogonek.app").await;

        // Setup student profile
        let student_id = create_test_user(&db, "student", "student@ogonek.app").await;

        // Create teacher-student relationship
        sqlx::query!(
            r#"
            INSERT INTO teacher_student (teacher_id, student_id)
            VALUES ($1, $2)
            "#,
            teacher_id,
            student_id
        )
        .execute(&db)
        .await
        .unwrap();
        let video_call_url = Some("https://zoom.us/j/pwd?=teacherroomurl".to_string());
        let telegram_id = Some("@teacher".to_string());

        sqlx::query!(
            r#"
           UPDATE profile
            SET
            telegram_id = COALESCE($2, telegram_id),
            video_call_url = COALESCE($3, video_call_url)
            WHERE user_id = $1
            "#,
            teacher_id,
            telegram_id,
            video_call_url
        )
        .execute(&db)
        .await
        .unwrap();

        // Test params for student
        let params = ProfileParams {
            is_student: "true".to_string(),
        };

        // Call function
        let result = find_by_id(&db, &student_id, &params).await;

        // Assertions
        assert!(result.is_ok());
        let profile_with_ts = result.unwrap();
        assert_eq!(profile_with_ts.profile.user_id, student_id);

        let teacher_data = profile_with_ts.teacher_data.unwrap();
        assert_eq!(teacher_data.teacher_telegram_id, telegram_id);
        assert_eq!(teacher_data.teacher_video_call_url, video_call_url);
    }

    #[sqlx::test]
    async fn test_find_by_id_student_without_teacher(db: PgPool) {
        // Setup student profile without teacher relationship
        let student_id = create_test_user(&db, "student", "student@ogonek.app").await;

        let params = ProfileParams {
            is_student: "true".to_string(),
        };

        let result = find_by_id(&db, &student_id, &params).await;

        assert!(result.is_ok());
        let profile_with_ts = result.unwrap();
        assert_eq!(profile_with_ts.profile.user_id, student_id);
        assert!(profile_with_ts.teacher_data.is_none());
    }

    #[sqlx::test]
    async fn test_upsert_new_profile(db: PgPool) {
        let user_id = create_test_user(&db, "update", "update@ogonek.app").await;
        let update = ProfileUpdate {
            video_call_url: Some("https://zoom.us/j/12345678901?pwd=new_user".to_string()),
            avatar_url: Some("https://example.com/new_avatar.png".to_string()),
            telegram_id: Some("@newuser".to_string()),
        };

        // Call upsert function
        let result = upsert(&db, &user_id, &update).await;
        assert!(result.is_ok());

        // Verify the profile was inserted
        let profile = sqlx::query_as!(Profile, "SELECT * FROM profile WHERE user_id = $1", user_id)
            .fetch_one(&db)
            .await
            .unwrap();

        assert_eq!(profile.user_id, user_id);
        assert_eq!(profile.video_call_url, update.video_call_url);
        assert_eq!(profile.avatar_url, update.avatar_url);
        assert_eq!(profile.telegram_id, update.telegram_id);
    }

    #[sqlx::test]
    async fn test_upsert_existing_profile_full_update(db: PgPool) {
        let user_id = create_test_user(&db, "full_update", "full_update@ogonek.app").await;

        // Update with new values
        let update = ProfileUpdate {
            video_call_url: Some("https://zoom.us/j/12345678901?pwd=updated".to_string()),
            avatar_url: Some("https://example.com/updated_avatar.png".to_string()),
            telegram_id: Some("@updateduser".to_string()),
        };

        let result = upsert(&db, &user_id, &update).await;
        assert!(result.is_ok());

        // Verify the profile was updated
        let profile = sqlx::query_as!(Profile, "SELECT * FROM profile WHERE user_id = $1", user_id)
            .fetch_one(&db)
            .await
            .unwrap();

        assert_eq!(profile.video_call_url, update.video_call_url);
        assert_eq!(profile.avatar_url, update.avatar_url);
        assert_eq!(profile.telegram_id, update.telegram_id);
    }

    #[sqlx::test]
    async fn test_upsert_with_all_none_values(db: PgPool) {
        let user_id = create_test_user(&db, "none", "none@ogonek.app").await;

        let update = ProfileUpdate {
            video_call_url: None,
            avatar_url: None,
            telegram_id: None,
        };

        let result = upsert(&db, &user_id, &update).await;
        assert!(result.is_ok());

        // Verify the profile was inserted with None values
        let profile = sqlx::query_as!(Profile, "SELECT * FROM profile WHERE user_id = $1", user_id)
            .fetch_one(&db)
            .await
            .unwrap();

        assert_eq!(profile.user_id, user_id);
        assert!(profile.video_call_url.is_none());
        assert!(profile.avatar_url.is_none());
        assert!(profile.telegram_id.is_none());
    }

    #[sqlx::test]
    async fn test_find_by_id_with_different_is_student_values(db: PgPool) {
        let user_id = create_test_user(&db, "student", "student@ogonek.app").await;

        // Test with is_student = "false"
        let params_false = ProfileParams {
            is_student: "false".to_string(),
        };
        let result = find_by_id(&db, &user_id, &params_false).await;
        assert!(result.is_ok());
        assert!(result.unwrap().teacher_data.is_none());

        // Test with is_student = "TRUE" (case shouldn't matter for the logic, but testing edge case)
        let params_other = ProfileParams {
            is_student: "TRUE".to_string(),
        };
        let result = find_by_id(&db, &user_id, &params_other).await;
        assert!(result.is_ok());
        // Should be None since it's not exactly "true"
        assert!(result.unwrap().teacher_data.is_none());
    }
}
