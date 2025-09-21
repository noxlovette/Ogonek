use chrono::{DateTime, TimeZone, Utc};
pub fn extract_occurrence_from_id(event_id: &str) -> Option<DateTime<Utc>> {
    if event_id.len() <= 21 {
        return None;
    }

    if let Some(pos) = event_id[21..].find('_') {
        let ts_str = &event_id[21 + pos + 1..];
        let ts: i64 = ts_str.parse().ok()?;
        return Utc.timestamp_opt(ts, 0).single();
    }

    None
}
pub fn remove_until_from_rrule(rrule: &str) -> String {
    regex::Regex::new(r";?UNTIL=[^;]*")
        .unwrap()
        .replace(rrule, "")
        .to_string()
}
pub mod rrule;
pub use rrule::*;
