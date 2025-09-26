pub mod accounts;
pub mod calendar;
pub mod content;
pub mod core;
pub mod preferences;
pub mod responses;
pub mod state;
pub use files::*;
mod files;
pub mod tracking;
pub use accounts::*;
pub use calendar::*;
pub use content::*;
pub mod third_party;
pub use core::*;
pub use preferences::*;
pub use responses::*;
pub use state::*;
pub use third_party::*;
pub use tracking::*;
pub mod datetime_serialization {
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Deserializer, Serializer};
    pub fn serialize<S>(dt: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let formatted = dt.format("%Y-%m-%dT%H:%M:%SZ").to_string();
        serializer.serialize_str(&formatted)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        DateTime::parse_from_rfc3339(&s)
            .map(|dt| dt.with_timezone(&Utc))
            .map_err(serde::de::Error::custom)
    }

    // For Option<DateTime<Utc>>
    pub mod option {
        use super::*;

        pub fn serialize<S>(dt: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match dt {
                Some(dt) => super::serialize(dt, serializer),
                None => serializer.serialize_none(),
            }
        }

        pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
        where
            D: Deserializer<'de>,
        {
            Option::<String>::deserialize(deserializer)?
                .map(|s| {
                    DateTime::parse_from_rfc3339(&s)
                        .map(|dt| dt.with_timezone(&Utc))
                        .map_err(serde::de::Error::custom)
                })
                .transpose()
        }
    }
}
