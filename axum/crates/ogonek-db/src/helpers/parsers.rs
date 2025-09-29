use std::collections::HashSet;

use chrono::{DateTime, NaiveDate, Utc};

use crate::error::DbError;

// Helper function pour parser les EXDATE depuis TEXT[]
pub fn parse_exdates(
    exdate_array: &Option<Vec<String>>,
) -> Result<HashSet<DateTime<Utc>>, DbError> {
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
pub fn parse_date_flexible(date_str: &str) -> Result<DateTime<Utc>, DbError> {
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
