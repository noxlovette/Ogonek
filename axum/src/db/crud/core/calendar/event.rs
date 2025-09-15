use sqlx::PgPool;

use crate::{
    db::error::DbError,
    types::{
        CalendarEvent, CalendarEventCreate, CalendarEventUpdate, EventClass, EventStatus,
        EventTransp,
    },
};

/// Finds a calendar event by id
pub async fn find_by_id(db: &PgPool, event_id: &str) -> Result<CalendarEvent, DbError> {
    let event = sqlx::query_as!(
        CalendarEvent,
        r#"
        SELECT 
            id,
            uid,
            created_at,
            updated_at,
            calendar_id,
            summary,
            description,
            location,
            url,
            dtstart,
            dtend,
            all_day,
            timezone,
            rrule,
            rdate,
            exdate,
            recurrence_id,
            status as "status: EventStatus",
            class as "class: EventClass",
            transp as "transp: EventTransp",
            priority,
            categories,
            organiser_email,
            organiser_name,
            sequence,
            dtstamp,
            etag,
            deleted_at
        FROM calendar_events
        WHERE id = $1 AND deleted_at IS NULL
        "#,
        event_id
    )
    .fetch_one(db)
    .await?;

    Ok(event)
}

/// Finds all events for a calendar
pub async fn find_by_calendar_id(
    db: &PgPool,
    calendar_id: &str,
) -> Result<Vec<CalendarEvent>, DbError> {
    let events = sqlx::query_as!(
        CalendarEvent,
        r#"
        SELECT 
            id,
            uid,
            created_at,
            updated_at,
            calendar_id,
            summary,
            description,
            location,
            url,
            dtstart,
            dtend,
            all_day,
            timezone,
            rrule,
            rdate,
            exdate,
            recurrence_id,
            status as "status: EventStatus",
            class as "class: EventClass",
            transp as "transp: EventTransp",
            priority,
            categories,
            organiser_email,
            organiser_name,
            sequence,
            dtstamp,
            etag,
            deleted_at
        FROM calendar_events
        WHERE calendar_id = $1 AND deleted_at IS NULL
        ORDER BY dtstart ASC
        "#,
        calendar_id
    )
    .fetch_all(db)
    .await?;

    Ok(events)
}
pub async fn find_by_date(
    db: &PgPool,
    day: chrono::NaiveDate,
) -> Result<Vec<CalendarEvent>, DbError> {
    let events = sqlx::query_as!(
        CalendarEvent,
        r#"
        SELECT 
            id, uid, created_at, updated_at, calendar_id,
            summary, description, location, url,
            dtstart, dtend, all_day, timezone,
            rrule, rdate, exdate, recurrence_id,
            status as "status: EventStatus",
            class as "class: EventClass", 
            transp as "transp: EventTransp",
            priority, categories,
            organiser_email, organiser_name,
            sequence, dtstamp, etag, deleted_at
        FROM calendar_events
        WHERE deleted_at IS NULL
        AND (
            dtstart::date = $1 OR 
            dtend::date = $1 OR
            (dtstart::date <= $1 AND dtend::date >= $1)
        )
        ORDER BY dtstart ASC
        "#,
        day,
    )
    .fetch_all(db)
    .await?;
    Ok(events)
}

/// Soft deletes a calendar event (sets deleted_at timestamp)
pub async fn delete(db: &PgPool, event_id: &str) -> Result<(), DbError> {
    sqlx::query!(
        r#"
        UPDATE calendar_events
        SET deleted_at = NOW()
        WHERE id = $1
        "#,
        event_id
    )
    .execute(db)
    .await?;

    Ok(())
}
pub async fn update(
    db: &PgPool,
    event_id: &str,
    update: &CalendarEventUpdate,
) -> Result<(), DbError> {
    sqlx::query!(
        r#"
        UPDATE calendar_events 
        SET
            summary = COALESCE($1, summary),
            description = COALESCE($2, description),
            location = COALESCE($3, location),
            url = COALESCE($4, url),
            dtstart = COALESCE($5, dtstart),
            dtend = COALESCE($6, dtend),
            all_day = COALESCE($7, all_day),
            timezone = COALESCE($8, timezone),
            rrule = COALESCE($9, rrule),
            rdate = COALESCE($10::TEXT[], rdate),
            exdate = COALESCE($11::TEXT[], exdate),
            recurrence_id = COALESCE($12, recurrence_id),
            status = COALESCE($13::VARCHAR, status),
            class = COALESCE($14::VARCHAR, class),
            transp = COALESCE($15::VARCHAR, transp),
            priority = COALESCE($16, priority),
            categories = COALESCE($17::TEXT[], categories),
            organiser_email = COALESCE($18, organiser_email),
            organiser_name = COALESCE($19, organiser_name),
            sequence = COALESCE($20, sequence),
            dtstamp = COALESCE($21, dtstamp),
            etag = COALESCE($22, etag),
            updated_at = NOW()
        WHERE id = $23 AND deleted_at IS NULL
        "#,
        update.summary,
        update.description,
        update.location,
        update.url,
        update.dtstart,
        update.dtend,
        update.all_day,
        update.timezone,
        update.rrule,
        update.rdate.as_ref().map(|v| v.as_slice()), // Vec<String> → &[String]
        update.exdate.as_ref().map(|v| v.as_slice()),
        update.recurrence_id,
        update.status.as_ref().map(|s| s.to_string().to_lowercase()), // Clean enum → string
        update.class.as_ref().map(|c| c.to_string().to_lowercase()),
        update.transp.as_ref().map(|t| t.to_string().to_lowercase()),
        update.priority,
        update.categories.as_ref().map(|v| v.as_slice()),
        update.organiser_email,
        update.organiser_name,
        update.sequence,
        update.dtstamp,
        update.etag,
        event_id
    )
    .execute(db)
    .await?;
    Ok(())
}

/// Creates a calendar event
pub async fn create(
    db: &PgPool,
    uid: &str,
    calendar_id: &str,
    create: CalendarEventCreate,
) -> Result<String, DbError> {
    let id = sqlx::query_scalar!(
        r#"
        INSERT INTO calendar_events (
            id, calendar_id, uid, summary, dtstart, dtend
        )
        VALUES (
            $1, $2, $3, $4, $5, $6
        )
        RETURNING id
        "#,
        nanoid::nanoid!(),
        calendar_id,
        uid,
        create.summary,
        create.dtstart,
        create.dtend,
    )
    .fetch_one(db)
    .await?;

    Ok(id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::crud::core::calendar::calendar;
    use crate::tests::create_test_user;
    use crate::types::{
        CalendarCreate, CalendarEventCreate, CalendarEventUpdate, EventClass, EventStatus,
        EventTransp,
    };
    use chrono::Utc;

    async fn create_test_calendar(db: &PgPool, user_id: &str, name: &str) -> String {
        let calendar_create = CalendarCreate {
            name: name.to_string(),
        };
        calendar::create(db, user_id, calendar_create)
            .await
            .unwrap()
    }

    async fn create_test_event(db: &PgPool, calendar_id: &str, summary: &str) -> String {
        let event_create = CalendarEventCreate {
            summary: summary.to_string(),
            dtstart: Utc::now(),
            dtend: Some(Utc::now()),
        };
        let uid = nanoid::nanoid!();
        create(db, &uid, calendar_id, event_create).await.unwrap()
    }

    #[sqlx::test]
    async fn test_create_event(db: PgPool) {
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let calendar_id = create_test_calendar(&db, &user_id, "Test Calendar").await;

        let event_create = CalendarEventCreate {
            summary: "Test Event".to_string(),
            dtstart: Utc::now(),
            dtend: Some(Utc::now()),
        };

        let uid = nanoid::nanoid!();
        let result = create(&db, &uid, &calendar_id, event_create).await;
        assert!(result.is_ok());

        let event_id = result.unwrap();
        assert!(!event_id.is_empty());
    }

    #[sqlx::test]
    async fn test_find_by_id_success(db: PgPool) {
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let calendar_id = create_test_calendar(&db, &user_id, "Test Calendar").await;
        let event_id = create_test_event(&db, &calendar_id, "Test Event").await;

        let result = find_by_id(&db, &event_id).await;
        assert!(result.is_ok());

        let event = result.unwrap();
        assert_eq!(event.id, event_id);
        assert_eq!(event.summary, "Test Event");
        assert_eq!(event.calendar_id, calendar_id);
    }

    #[sqlx::test]
    async fn test_find_by_id_not_found(db: PgPool) {
        let non_existent_id = "non-existent-id";
        let result = find_by_id(&db, non_existent_id).await;
        assert!(result.is_err());
    }

    #[sqlx::test]
    async fn test_find_by_calendar_id(db: PgPool) {
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let calendar_id = create_test_calendar(&db, &user_id, "Test Calendar").await;

        create_test_event(&db, &calendar_id, "Event 1").await;
        create_test_event(&db, &calendar_id, "Event 2").await;
        create_test_event(&db, &calendar_id, "Event 3").await;

        let result = find_by_calendar_id(&db, &calendar_id).await;
        assert!(result.is_ok());

        let events = result.unwrap();
        assert_eq!(events.len(), 3);
    }

    #[sqlx::test]
    async fn test_find_by_calendar_id_empty_result(db: PgPool) {
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let calendar_id = create_test_calendar(&db, &user_id, "Empty Calendar").await;

        let result = find_by_calendar_id(&db, &calendar_id).await;
        assert!(result.is_ok());

        let events = result.unwrap();
        assert_eq!(events.len(), 0);
    }

    #[sqlx::test]
    async fn test_update_event(db: PgPool) {
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let calendar_id = create_test_calendar(&db, &user_id, "Test Calendar").await;
        let event_id = create_test_event(&db, &calendar_id, "Original Summary").await;

        let update_data = CalendarEventUpdate {
            summary: Some("Updated Summary".to_string()),
            description: Some("Updated description".to_string()),
            location: Some("Updated location".to_string()),
            status: Some(EventStatus::Confirmed),
            class: Some(EventClass::Public),
            transp: Some(EventTransp::Opaque),
            ..Default::default()
        };

        let result = update(&db, &event_id, &update_data).await;
        assert!(result.is_ok());

        let event = find_by_id(&db, &event_id).await.unwrap();
        assert_eq!(event.summary, "Updated Summary");
        assert_eq!(event.description, Some("Updated description".to_string()));
        assert_eq!(event.location, Some("Updated location".to_string()));
        assert_eq!(event.status, EventStatus::Confirmed);
        assert_eq!(event.class, EventClass::Public);
        assert_eq!(event.transp, EventTransp::Opaque);
    }

    #[sqlx::test]
    async fn test_update_partial(db: PgPool) {
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let calendar_id = create_test_calendar(&db, &user_id, "Test Calendar").await;
        let event_id = create_test_event(&db, &calendar_id, "Original Summary").await;

        let update_data = CalendarEventUpdate {
            summary: Some("Updated Summary".to_string()),
            description: None,
            location: None,
            ..Default::default()
        };

        let result = update(&db, &event_id, &update_data).await;
        assert!(result.is_ok());

        let event = find_by_id(&db, &event_id).await.unwrap();
        assert_eq!(event.summary, "Updated Summary");
    }

    #[sqlx::test]
    async fn test_delete_event(db: PgPool) {
        let user_id = create_test_user(&db, "testuser", "test@example.com").await;
        let calendar_id = create_test_calendar(&db, &user_id, "Test Calendar").await;
        let event_id = create_test_event(&db, &calendar_id, "Event to Delete").await;

        let result = delete(&db, &event_id).await;
        assert!(result.is_ok());

        let find_result = find_by_id(&db, &event_id).await;
        assert!(find_result.is_err());
    }

    #[sqlx::test]
    async fn test_delete_non_existent(db: PgPool) {
        let non_existent_id = "non-existent-id";
        let result = delete(&db, non_existent_id).await;
        assert!(result.is_ok());
    }
}
