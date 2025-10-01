use std::collections::{HashMap, HashSet};

use chrono::{DateTime, Utc};
use sqlx::PgPool;

use crate::{crud::core::calendar::cal::read_calendar_id, error::DbError};

use crate::helpers::{
    OCCURRENCE_SEPARATOR, RRule, extract_id_and_occurence, parse_date_flexible, parse_exdates,
};
use ogonek_types::{
    CalendarRole, EventClass, EventDB, EventDBFull, EventFull, EventSmall, EventStatus, EventTransp,
};

/// Reads a calendar event by id (supports virtual instances)
pub async fn read_one(db: &PgPool, event_id: String) -> Result<EventFull, DbError> {
    let (master_id, occurrence_date) = extract_id_and_occurence(event_id.clone());

    let mut tx = db.begin().await?;

    let mut master = read_one_internal(&mut *tx, &master_id).await?;

    // If this is a virtual instance, modify the dates
    if let Some(occurrence_dt) = occurrence_date {
        // Calculate the duration of the original event
        let original_duration = master.dtend_time.map(|end| end - master.dtstart_time);

        // Update the start time to the occurrence date
        master.dtstart_time = occurrence_dt;

        // Update the end time to maintain the same duration
        if let Some(duration) = original_duration {
            master.dtend_time = Some(occurrence_dt + duration);
        }

        // Update the ID to the virtual instance ID
        master.id = event_id;
    }

    tx.commit().await?;
    Ok(master.into())
}

/// Reads a calendar event by id
pub(super) async fn read_one_internal(
    db: impl sqlx::Executor<'_, Database = sqlx::Postgres>,
    event_id: &str,
) -> Result<EventDBFull, DbError> {
    let event = sqlx::query_as!(
        EventDBFull,
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
            dtstart_time,
            dtend_time,
            dtend_tz,
            dtstart_tz,
            rrule,
            rdate,
            exdate,
            recurrence_id,
            status AS "status!: EventStatus",
            class AS "class!: EventClass", 
            transp AS "transp!: EventTransp",
            priority,
            categories,
            sequence,
            etag,
            deleted_at,
            caldav_href,
            content_type
        FROM calendar_events 
        WHERE id = $1 AND deleted_at IS NULL
        "#,
        event_id
    )
    .fetch_one(db)
    .await?;

    Ok(event)
}

/// Reads all events for a calendar within a specific time frame
pub(super) async fn read_all_internal(
    db: &PgPool,
    user_id: &str,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Result<Vec<EventDB>, DbError> {
    let mut tx = db.begin().await?;

    let calendar_id = read_calendar_id(&mut *tx, user_id).await?;
    let events = sqlx::query_as!(
        EventDB,
        r#"
        SELECT 
            id,
            uid,
            summary,
            location,
            dtstart_time,
            dtend_time,
            rdate,
            status AS "status!: EventStatus",
            exdate,
            recurrence_id,
            rrule
        FROM calendar_events
        WHERE calendar_id = $1 
            AND deleted_at IS NULL
            AND (
                (rrule IS NOT NULL AND recurrence_id IS NULL) -- Master events
            OR (dtstart_time BETWEEN $2 AND $3) -- Single events in range
            OR (recurrence_id IS NOT NULL) -- All exceptions (we'll filter later)
            )
        ORDER BY dtstart_time ASC
        "#,
        calendar_id,
        start,
        end
    )
    .fetch_all(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(events)
}

/// The impl for student roles
pub async fn read_events_as_invitee(
    pool: &PgPool,
    user_id: &str,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Result<Vec<EventDB>, sqlx::Error> {
    sqlx::query_as!(
        EventDB,
        r#"
        SELECT 
        e.id,
        e.uid,
        e.summary,
        e.location,
        e.dtstart_time,
        e.dtend_time,
        e.rdate,
        e.status AS "status!: EventStatus",
        e.exdate,
        e.recurrence_id,
        e.rrule
        FROM calendar_events e
        INNER JOIN event_attendees ea ON e.id = ea.event_id
        WHERE ea.user_id = $1 

        AND ea.status != 'DECLINED'
          AND e.deleted_at IS NULL
            AND (
                (e.rrule IS NOT NULL AND e.recurrence_id IS NULL)
            OR (e.dtstart_time BETWEEN $2 AND $3)
            OR (e.recurrence_id IS NOT NULL) 
            )
        ORDER BY e.dtstart_time ASC
        "#,
        user_id,
        start,
        end
    )
    .fetch_all(pool)
    .await
}
/// Public function that returns an array of events
pub async fn read_all(
    db: &PgPool,
    user_id: &str,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    role: CalendarRole,
) -> Result<Vec<EventSmall>, DbError> {
    let events = match role {
        CalendarRole::Teacher => read_all_internal(db, user_id, start, end).await?,
        CalendarRole::Student => read_events_as_invitee(db, user_id, start, end).await?,
    };

    let masters: Vec<_> = events
        .iter()
        .filter(|e| e.recurrence_id.is_none())
        .collect();

    let mut exceptions: HashMap<String, Vec<&EventDB>> = HashMap::new();
    for e in events.iter().filter(|e| e.recurrence_id.is_some()) {
        exceptions.entry(e.uid.clone()).or_default().push(e);
    }

    let mut calendar_events: Vec<EventSmall> = Vec::new();

    for master in masters {
        match RRule::parse(master.rrule.clone())? {
            Some(rrule) => {
                let mut all_occurrences = HashSet::new();

                // Generate regular RRULE occurrences
                let rrule_occurrences = rrule.generate_occurrences(master.dtstart_time, start, end);
                all_occurrences.extend(rrule_occurrences);

                // Always include the original dtstart_time if it's in range
                // This ensures the first occurrence isn't lost when adding recurrence
                if master.dtstart_time >= start && master.dtstart_time <= end {
                    all_occurrences.insert(master.dtstart_time);
                }

                // Add RDATE occurrences (additional dates)
                if let Some(rdate_list) = &master.rdate {
                    for date_str in rdate_list {
                        let rdate = parse_date_flexible(date_str)?;
                        if rdate >= start && rdate <= end {
                            all_occurrences.insert(rdate);
                        }
                    }
                }

                // Parse EXDATE array from master event
                let exdates = parse_exdates(&master.exdate)?;

                for occurrence in all_occurrences {
                    // Skip if this occurrence is in EXDATE
                    if exdates.contains(&occurrence) {
                        continue;
                    }

                    // Check if this occurrence has an exception (modified instance)
                    let has_exception = exceptions
                        .get(&master.uid)
                        .map(|excs| excs.iter().any(|exc| exc.recurrence_id == Some(occurrence)))
                        .unwrap_or(false);

                    if !has_exception {
                        // Virtual instances
                        calendar_events.push(EventSmall {
                            db_data: EventDB {
                                id: format!(
                                    "{}{}{}",
                                    master.id,
                                    OCCURRENCE_SEPARATOR,
                                    occurrence.timestamp()
                                ),
                                uid: master.uid.clone(),
                                rrule: master.rrule.clone(),
                                status: master.status.clone(),
                                rdate: master.rdate.clone(),
                                exdate: master.rdate.clone(),
                                recurrence_id: None,
                                summary: master.summary.clone(),
                                dtstart_time: occurrence,
                                location: master.location.clone(),
                                dtend_time: master
                                    .dtend_time
                                    .map(|end| occurrence + (end - master.dtstart_time)),
                            },
                            is_recurring: true,
                            is_exception: false,
                        });
                    }
                }
            }
            None => {
                // Non-recurring event
                calendar_events.push(EventSmall {
                    db_data: EventDB {
                        id: master.id.clone(),
                        uid: master.uid.clone(),
                        recurrence_id: None,
                        rrule: None,
                        status: master.status.clone(),
                        rdate: None,
                        exdate: None,
                        summary: master.summary.clone(),
                        location: master.location.clone(),
                        dtstart_time: master.dtstart_time,
                        dtend_time: master.dtend_time,
                    },
                    is_recurring: false,
                    is_exception: false,
                });
            }
        }
    }

    // Add exception instances (modified occurrences)
    for (uid, exception_list) in exceptions {
        for exception in exception_list {
            if exception.dtstart_time >= start && exception.dtstart_time <= end {
                calendar_events.push(EventSmall {
                    db_data: EventDB {
                        id: exception.id.clone(),
                        uid: uid.clone(),
                        recurrence_id: exception.recurrence_id,
                        status: exception.status.clone(),
                        rrule: None,
                        exdate: None,
                        rdate: None,
                        summary: exception.summary.clone(),
                        location: exception.location.clone(),
                        dtstart_time: exception.dtstart_time,
                        dtend_time: exception.dtend_time,
                    },
                    is_recurring: true,
                    is_exception: true,
                });
            }
        }
    }

    Ok(calendar_events)
}
