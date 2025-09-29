use std::collections::{HashMap, HashSet};
services::calendar::{extract_id_and_occurence, remove_until_from_rrule},


    let new_rrule = truncate_master(tx, master, &occurrence_date).await?;
pub(super) async fn truncate_master(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    master: &EventDBFull,
    occurrence_date: &DateTime<Utc>,
) -> Result<String, DbError> {
    let rrule_str = master.rrule.as_ref().ok_or(DbError::NotRecurring)?;
    let until_date = *occurrence_date - Duration::seconds(1);

    // Format iCal compact conforme à la spec
    let until_formatted = until_date.format("%Y%m%dT%H%M%SZ").to_string();

    let truncated_rrule = if rrule_str.contains("UNTIL=") {
        // Replace existing UNTIL
        let re = regex::Regex::new(r"UNTIL=[^;]*").unwrap();
        re.replace(rrule_str, &format!("UNTIL={}", until_formatted))
            .to_string()
    } else {
        // Add UNTIL
        format!("{};UNTIL={}", rrule_str, until_formatted)
    };

    sqlx::query!(
        r#"
        UPDATE calendar_events 
        SET rrule = $1,
            sequence = sequence + 1
        WHERE id = $2
        "#,
        truncated_rrule,
        master.id
    )
    .execute(&mut **tx)
    .await?;

    Ok(remove_until_from_rrule(rrule_str))
}



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