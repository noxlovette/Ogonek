use sqlx::PgPool;

use crate::{
    db::error::DbError,
    types::{
        EventAttendee, EventAttendeeCreate, EventAttendeeRole, EventAttendeeStatus,
        EventAttendeeUpdate,
    },
};

/// Finds an event attendee by id
pub async fn find_by_id(db: &PgPool, attendee_id: &str) -> Result<EventAttendee, DbError> {
    let attendee = sqlx::query_as!(
        EventAttendee,
        r#"
        SELECT 
            id,
            event_id,
            email,
            name,
            role as "role: EventAttendeeRole",
            status as "status: EventAttendeeStatus",
            rsvp,
            created_at,
            updated_at
        FROM event_attendees
        WHERE id = $1
        "#,
        attendee_id
    )
    .fetch_one(db)
    .await?;

    Ok(attendee)
}

/// Finds all attendees for an event
pub async fn find_by_event_id(db: &PgPool, event_id: &str) -> Result<Vec<EventAttendee>, DbError> {
    let attendees = sqlx::query_as!(
        EventAttendee,
        r#"
        SELECT 
            id,
            event_id,
            email,
            name,
            role as "role: _",
            status as "status: _",
            rsvp,
            created_at,
            updated_at
        FROM event_attendees
        WHERE event_id = $1
        "#,
        event_id
    )
    .fetch_all(db)
    .await?;

    Ok(attendees)
}

/// Deletes an event attendee
pub async fn delete(db: &PgPool, attendee_id: &str) -> Result<(), DbError> {
    sqlx::query!(
        r#"
        DELETE FROM event_attendees
        WHERE id = $1
        "#,
        attendee_id
    )
    .execute(db)
    .await?;

    Ok(())
}

/// Updates an event attendee
pub async fn update(
    db: &PgPool,
    attendee_id: &str,
    update: &EventAttendeeUpdate,
) -> Result<(), DbError> {
    sqlx::query!(
        r#"
        UPDATE event_attendees 
        SET
            email = COALESCE($1, email),
            name = COALESCE($2, name),
            role = COALESCE($3, role),
            status = COALESCE($4, status),
            rsvp = COALESCE($5, rsvp)
        WHERE id = $6
        "#,
        update.email,
        update.name,
        update.role.as_ref().map(|r| r.to_string()),
        update.status.as_ref().map(|s| s.to_string()),
        update.rsvp,
        attendee_id
    )
    .execute(db)
    .await?;

    Ok(())
}

/// Creates an event attendee
pub async fn create(
    db: &PgPool,
    event_id: &str,
    create: EventAttendeeCreate,
) -> Result<String, DbError> {
    let id = sqlx::query_scalar!(
        r#"
        INSERT INTO event_attendees (id, event_id, email, name)
        VALUES ($1, $2, $3, $4)
        RETURNING id
        "#,
        nanoid::nanoid!(),
        event_id,
        create.email,
        create.name,
    )
    .fetch_one(db)
    .await?;

    Ok(id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::crud::core::calendar::{calendar, event};
    use crate::tests::create_test_user;
    use crate::types::{CalendarCreate, CalendarEventCreate, EventAttendeeCreate, EventAttendeeUpdate, EventAttendeeRole, EventAttendeeStatus};
    use chrono::Utc;

    async fn create_test_calendar(db: &PgPool, user_id: &str, name: &str) -> String {
        let calendar_create = CalendarCreate {
            name: name.to_string(),
        };
        calendar::create(db, user_id, calendar_create).await.unwrap()
    }

    async fn create_test_event(db: &PgPool, calendar_id: &str, summary: &str) -> String {
        let event_create = CalendarEventCreate {
            summary: summary.to_string(),
            dtstart: Utc::now(),
            dtend: Some(Utc::now()),
        };
        let uid = nanoid::nanoid!();
        event::create(db, &uid, calendar_id, event_create).await.unwrap()
    }

    async fn create_test_attendee(db: &PgPool, event_id: &str, email: &str, name: Option<&str>) -> String {
        let attendee_create = EventAttendeeCreate {
            email: email.to_string(),
            name: name.map(|n| n.to_string()),
        };
        create(db, event_id, attendee_create).await.unwrap()
    }

    #[sqlx::test]
    async fn test_create_attendee(db: PgPool) {
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let calendar_id = create_test_calendar(&db, &user_id, "Test Calendar").await;
        let event_id = create_test_event(&db, &calendar_id, "Test Event").await;

        let attendee_create = EventAttendeeCreate {
            email: "attendee@example.com".to_string(),
            name: Some("Test Attendee".to_string()),
        };

        let result = create(&db, &event_id, attendee_create).await;
        assert!(result.is_ok());

        let attendee_id = result.unwrap();
        assert!(!attendee_id.is_empty());
    }

    #[sqlx::test]
    async fn test_find_by_id_success(db: PgPool) {
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let calendar_id = create_test_calendar(&db, &user_id, "Test Calendar").await;
        let event_id = create_test_event(&db, &calendar_id, "Test Event").await;
        let attendee_id = create_test_attendee(&db, &event_id, "test@example.com", Some("Test User")).await;

        let result = find_by_id(&db, &attendee_id).await;
        assert!(result.is_ok());

        let attendee = result.unwrap();
        assert_eq!(attendee.id, attendee_id);
        assert_eq!(attendee.email, "test@example.com");
        assert_eq!(attendee.name, Some("Test User".to_string()));
        assert_eq!(attendee.event_id, event_id);
    }

    #[sqlx::test]
    async fn test_find_by_id_not_found(db: PgPool) {
        let non_existent_id = "non-existent-id";
        let result = find_by_id(&db, non_existent_id).await;
        assert!(result.is_err());
    }

    #[sqlx::test]
    async fn test_find_by_event_id(db: PgPool) {
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let calendar_id = create_test_calendar(&db, &user_id, "Test Calendar").await;
        let event_id = create_test_event(&db, &calendar_id, "Test Event").await;
        
        create_test_attendee(&db, &event_id, "attendee1@example.com", Some("Attendee 1")).await;
        create_test_attendee(&db, &event_id, "attendee2@example.com", Some("Attendee 2")).await;
        create_test_attendee(&db, &event_id, "attendee3@example.com", None).await;

        let result = find_by_event_id(&db, &event_id).await;
        assert!(result.is_ok());

        let attendees = result.unwrap();
        assert_eq!(attendees.len(), 3);
    }

    #[sqlx::test]
    async fn test_find_by_event_id_empty_result(db: PgPool) {
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let calendar_id = create_test_calendar(&db, &user_id, "Test Calendar").await;
        let event_id = create_test_event(&db, &calendar_id, "Empty Event").await;

        let result = find_by_event_id(&db, &event_id).await;
        assert!(result.is_ok());

        let attendees = result.unwrap();
        assert_eq!(attendees.len(), 0);
    }

    #[sqlx::test]
    async fn test_update_attendee(db: PgPool) {
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let calendar_id = create_test_calendar(&db, &user_id, "Test Calendar").await;
        let event_id = create_test_event(&db, &calendar_id, "Test Event").await;
        let attendee_id = create_test_attendee(&db, &event_id, "original@example.com", Some("Original Name")).await;

        let update_data = EventAttendeeUpdate {
            email: Some("updated@example.com".to_string()),
            name: Some("Updated Name".to_string()),
            role: Some(EventAttendeeRole::Chair),
            status: Some(EventAttendeeStatus::Accepted),
            rsvp: Some(true),
        };

        let result = update(&db, &attendee_id, &update_data).await;
        assert!(result.is_ok());

        let attendee = find_by_id(&db, &attendee_id).await.unwrap();
        assert_eq!(attendee.email, "updated@example.com");
        assert_eq!(attendee.name, Some("Updated Name".to_string()));
        assert_eq!(attendee.role, EventAttendeeRole::Chair);
        assert_eq!(attendee.status, EventAttendeeStatus::Accepted);
        assert_eq!(attendee.rsvp, true);
    }

    #[sqlx::test]
    async fn test_update_partial(db: PgPool) {
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let calendar_id = create_test_calendar(&db, &user_id, "Test Calendar").await;
        let event_id = create_test_event(&db, &calendar_id, "Test Event").await;
        let attendee_id = create_test_attendee(&db, &event_id, "original@example.com", Some("Original Name")).await;

        let update_data = EventAttendeeUpdate {
            email: Some("updated@example.com".to_string()),
            name: None,
            role: None,
            status: None,
            rsvp: None,
        };

        let result = update(&db, &attendee_id, &update_data).await;
        assert!(result.is_ok());

        let attendee = find_by_id(&db, &attendee_id).await.unwrap();
        assert_eq!(attendee.email, "updated@example.com");
        assert_eq!(attendee.name, Some("Original Name".to_string()));
    }

    #[sqlx::test]
    async fn test_delete_attendee(db: PgPool) {
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let calendar_id = create_test_calendar(&db, &user_id, "Test Calendar").await;
        let event_id = create_test_event(&db, &calendar_id, "Test Event").await;
        let attendee_id = create_test_attendee(&db, &event_id, "delete@example.com", Some("To Be Deleted")).await;

        let result = delete(&db, &attendee_id).await;
        assert!(result.is_ok());

        let find_result = find_by_id(&db, &attendee_id).await;
        assert!(find_result.is_err());
    }

    #[sqlx::test]
    async fn test_delete_non_existent(db: PgPool) {
        let non_existent_id = "non-existent-id";
        let result = delete(&db, non_existent_id).await;
        assert!(result.is_ok());
    }
}
