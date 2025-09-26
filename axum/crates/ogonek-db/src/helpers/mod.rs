use crate::DbError;
use chrono::TimeZone;
use chrono::{DateTime, Utc};
use ogonek_types::User;
mod parsers;
mod rrule;

pub use parsers::*;
pub use rrule::*;

pub trait FromQuery: Sized {
    fn from_query_result(result: Vec<Self>) -> Result<Self, DbError> {
        result.into_iter().next().ok_or(DbError::TransactionFailed)
    }
}

impl FromQuery for User {}

const NANOID: usize = 21;
pub const OCCURRENCE_SEPARATOR: &str = "_occurrence_";
/// Extrait l'ID de base et l'occurrence virtuelle d'un event_id
/// Format attendu: "{nanoid}_occurrence_{timestamp}" ou juste "{nanoid}"
pub fn extract_id_and_occurence(event_id: String) -> (String, Option<DateTime<Utc>>) {
    // Si c'est juste un nanoid classique, pas d'occurrence virtuelle
    if event_id.len() <= NANOID {
        return (event_id, None);
    }

    // Cherche le separator après le nanoid
    let search_start = NANOID;
    if let Some(separator_pos) = event_id[search_start..].find(OCCURRENCE_SEPARATOR) {
        let actual_separator_pos = search_start + separator_pos;
        let base_id = event_id[..actual_separator_pos].to_string();

        // Extract timestamp part
        let timestamp_start = actual_separator_pos + OCCURRENCE_SEPARATOR.len();
        if timestamp_start < event_id.len() {
            let timestamp_str = &event_id[timestamp_start..];

            if let Ok(timestamp) = timestamp_str.parse::<i64>()
                && let Some(datetime) = Utc.timestamp_opt(timestamp, 0).single()
            {
                return (base_id, Some(datetime));
            }
        }
    }

    // Si on arrive ici, soit pas de separator, soit format invalide
    (event_id, None)
}

/// Remove the UNTIL part from an RRULE string according to RFC 5545
pub fn remove_until_from_rrule(rrule: &str) -> String {
    rrule
        .split(";")
        .filter(|part| !part.starts_with("UNTIL="))
        .collect::<Vec<_>>()
        .join(";")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_until() {
        assert_eq!(
            remove_until_from_rrule("FREQ=DAILY;UNTIL=20231231T235959Z;INTERVAL=2"),
            "FREQ=DAILY;INTERVAL=2"
        );
        assert_eq!(
            remove_until_from_rrule("UNTIL=20240101T000000Z;FREQ=WEEKLY"),
            "FREQ=WEEKLY"
        );
        assert_eq!(
            remove_until_from_rrule("FREQ=MONTHLY;INTERVAL=1"),
            "FREQ=MONTHLY;INTERVAL=1"
        );
    }

    #[test]
    fn test_extract_id_and_occurence_comprehensive() {
        // Test ID simple
        let simple_id = "abc123def456ghi789klm".to_string();
        let (id, occurrence) = extract_id_and_occurence(simple_id.clone());
        assert_eq!(id, simple_id);
        assert_eq!(occurrence, None);

        // Test avec occurrence virtuelle valide
        let timestamp = 1640995200i64; // 2022-01-01 00:00:00 UTC
        let base_id = "abc123def456ghi789klm";
        let virtual_id = format!("{}{}{}", base_id, OCCURRENCE_SEPARATOR, timestamp);
        let (id, occurrence) = extract_id_and_occurence(virtual_id);

        assert_eq!(id, base_id);
        assert_eq!(
            occurrence,
            Some(Utc.timestamp_opt(timestamp, 0).single().unwrap())
        );

        // Test avec timestamp invalide
        let invalid_virtual_id = format!("{}{}invalid_timestamp", base_id, OCCURRENCE_SEPARATOR);
        let (id, occurrence) = extract_id_and_occurence(invalid_virtual_id.clone());
        assert_eq!(id, invalid_virtual_id); // Retourne le truc complet si parsing fail
        assert_eq!(occurrence, None);

        // Test ID court
        let short_id = "short".to_string();
        let (id, occurrence) = extract_id_and_occurence(short_id.clone());
        assert_eq!(id, short_id);
        assert_eq!(occurrence, None);

        // Test edge case: nanoid exact mais pas d'occurrence
        let exact_nanoid = "a".repeat(NANOID);
        let (id, occurrence) = extract_id_and_occurence(exact_nanoid.clone());
        assert_eq!(id, exact_nanoid);
        assert_eq!(occurrence, None);
    }
}
