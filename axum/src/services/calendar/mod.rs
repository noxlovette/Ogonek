use chrono::{DateTime, TimeZone, Utc};

/// A way to handle virtual occurences
pub fn extract_id_and_occurence(event_id: String) -> (String, Option<DateTime<Utc>>) {
    if event_id.len() <= 21 {
        return (event_id, None);
    }

    if let Some(pos) = event_id[21..].find('_') {
        let ts_str = &event_id[21 + pos + 1..];
        let ts: i64 = ts_str.parse().ok()?;
        return (event_id, Utc.timestamp_opt(ts, 0).single());
    }

    (event_id, None)
}
pub fn remove_until_from_rrule(rrule: &str) -> String {
    regex::Regex::new(r";?UNTIL=[^;]*")
        .unwrap()
        .replace(rrule, "")
        .to_string()
}
pub mod rrule;
pub use rrule::*;
