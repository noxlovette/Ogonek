use std::collections::{HashMap, HashSet};

use chrono::{DateTime, NaiveDate, Utc};
use sqlx::PgPool;

use crate::{
    db::{crud::core::calendar::calendar::read_calendar_id, error::DbError},
    services::calendar::{OCCURRENCE_SEPARATOR, RRule},
    types::{EventClass, EventDB, EventFull, EventSmall, EventStatus, EventTransp},
};

/// Reads a calendar event by id
pub async fn read_one(
    db: impl sqlx::Executor<'_, Database = sqlx::Postgres>,
    event_id: &str,
) -> Result<EventFull, DbError> {
    let event = sqlx::query_as!(
        EventFull,
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
async fn read_all_master(
    db: &PgPool,
    user_id: &str,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Result<Vec<EventDB>, DbError> {
    let mut tx = db.begin().await?;

    let calendar_id = read_calendar_id(&mut *tx, &user_id).await?;
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
pub async fn read_all(
    db: &PgPool,
    user_id: &str,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Result<Vec<EventSmall>, DbError> {
    let events = read_all_master(db, user_id, start, end).await?;

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

// Helper function pour parser les EXDATE depuis TEXT[]
fn parse_exdates(exdate_array: &Option<Vec<String>>) -> Result<HashSet<DateTime<Utc>>, DbError> {
    let mut exdates = HashSet::new();

    if let Some(exdate_list) = exdate_array {
        for date_str in exdate_list {
            let date_str = date_str.trim();
            if !date_str.is_empty() {
                // Support plusieurs formats de dates
                let parsed_date = parse_date_flexible(date_str)?;
                exdates.insert(parsed_date);
            }
        }
    }

    Ok(exdates)
}

// Parser flexible pour différents formats de dates
fn parse_date_flexible(date_str: &str) -> Result<DateTime<Utc>, DbError> {
    // Format ISO date seule (2024-11-11)
    if let Ok(naive_date) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        return Ok(naive_date.and_hms_opt(0, 0, 0).unwrap().and_utc());
    }

    // Format avec temps (2024-11-11T10:00:00Z)
    if let Ok(dt) = DateTime::parse_from_rfc3339(date_str) {
        return Ok(dt.with_timezone(&Utc));
    }

    // Format iCal (20241111T100000Z)
    if let Ok(dt) = DateTime::parse_from_str(date_str, "%Y%m%dT%H%M%SZ") {
        return Ok(dt.with_timezone(&Utc));
    }

    // Format iCal date seule (20241111)
    if let Ok(naive_date) = NaiveDate::parse_from_str(date_str, "%Y%m%d") {
        return Ok(naive_date.and_hms_opt(0, 0, 0).unwrap().and_utc());
    }

    Err(DbError::ParseError(format!(
        "Unsupported date format: {}",
        date_str
    )))
}
