use chrono::{DateTime, TimeZone, Utc};

const NANOID: usize = 21;

/// A way to handle virtual occurences
pub fn extract_id_and_occurence(event_id: String) -> (String, Option<DateTime<Utc>>) {
    if event_id.len() <= NANOID {
        return (event_id, None);
    }

    if let Some(pos) = event_id[NANOID..].find('_') {
        let ts_str = &event_id[NANOID + pos + 1..];
        if let Ok(ts) = ts_str.parse::<i64>() {
            return (event_id, Utc.timestamp_opt(ts, 0).single());
        } else {
            return (event_id, None);
        }
    }

    (event_id, None)
}

pub mod rrule;
pub use rrule::*;
/// Remove the UNTIL part from an RRULE string according to RFC 5545.
///
/// Example:
/// ```
/// let rrule = "FREQ=DAILY;UNTIL=20231231T235959Z;INTERVAL=2";
/// assert_eq!(remove_until_from_rrule(rrule), "FREQ=DAILY;INTERVAL=2");
/// ```
pub fn remove_until_from_rrule(rrule: &str) -> String {
    regex::Regex::new(r";?UNTIL=[^;]*")
        .unwrap()
        .replace(rrule, "")
        .to_string()
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
}
