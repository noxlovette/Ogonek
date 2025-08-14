pub mod files;
pub mod flashcards;
pub mod lessons;
pub mod notifications;
pub mod preferences;
pub mod profiles;
pub mod s3;
pub mod students;
pub mod tasks;
pub mod tracking;
pub mod users;

pub use files::*;
pub use flashcards::*;
pub use lessons::*;
pub use notifications::*;
pub use preferences::*;
pub use profiles::*;
pub use s3::*;
use serde::Serialize;
pub use students::*;
pub use tasks::*;
pub use tracking::*;
pub use users::*;
use utoipa::ToSchema;

/// Generic response that stores paginated data
#[derive(Debug, Serialize, ToSchema)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

/// Generic wrapper with the badge count in it
#[derive(Serialize, ToSchema)]
pub struct BadgeWrapper<T> {
    pub data: Vec<T>,
    pub count: i64,
}

// Explicit structs for OpenAPI
#[derive(Debug, Serialize, ToSchema)]
pub struct PaginatedTasks {
    pub data: Vec<TaskSmall>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PaginatedLessons {
    pub data: Vec<LessonSmall>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

// Same for badge wrappers
#[derive(Debug, Serialize, ToSchema)]
pub struct BadgeWrapperTasks {
    pub data: Vec<TaskSmall>,
    pub count: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BadgeWrapperLessons {
    pub data: Vec<LessonSmall>,
    pub count: i64,
}

#[derive(Serialize, ToSchema)]
pub struct BadgeWrapperDecks {
    pub data: Vec<DeckSmall>,
    pub count: i64,
}

// Conversion helpers
impl From<PaginatedResponse<TaskSmall>> for PaginatedTasks {
    fn from(paginated: PaginatedResponse<TaskSmall>) -> Self {
        Self {
            data: paginated.data,
            total: paginated.total,
            page: paginated.page,
            per_page: paginated.per_page,
        }
    }
}

impl From<PaginatedResponse<LessonSmall>> for PaginatedLessons {
    fn from(paginated: PaginatedResponse<LessonSmall>) -> Self {
        Self {
            data: paginated.data,
            total: paginated.total,
            page: paginated.page,
            per_page: paginated.per_page,
        }
    }
}

/// Simply contains one string, the created ID
#[derive(Debug, Serialize, ToSchema)]
pub struct CreationId {
    pub id: String,
}

/// A big response that powers the dashboard view
#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DashboardData {
    pub students: Vec<Student>,
    pub lessons: BadgeWrapperLessons,
    pub tasks: BadgeWrapperTasks,
    pub decks: BadgeWrapperDecks,
    pub user: User,
    pub profile: ProfileWithTS,
    pub activity: Vec<ActivityLog>,
    pub learn: LearnDataDashboard,
    pub preferences: UserPreferences,
}

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
