use diesel::prelude::*;
use uuid::Uuid;
use serde::{Deserialize, Serialize};


#[derive(Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::tasks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Task {
    pub id: String,
    pub title: String,
    pub content: String,
    pub priority: i16,
    pub completed: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub due_date: Option<chrono::DateTime<chrono::Utc>>,
    pub file: Option<String>,
    pub assignee_id: Uuid,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::tasks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewTask<'a> {
    pub id: String, // ULID as String
    pub title: &'a str,
    pub content: &'a str,
    pub priority: &'a i16,
    pub completed: &'a bool,
    pub due_date: Option<&'a chrono::DateTime<chrono::Utc>>,
    pub file: Option<&'a str>,
    pub assignee_id: &'a Uuid,
}

pub struct UpdateTask {
    pub title: Option<String>,
    pub content: Option<String>,
    pub priority: Option<i16>,
    pub completed: Option<bool>,
    pub due_date: Option<chrono::DateTime<chrono::Utc>>,
    pub file: Option<String>,
    pub assignee_id: Option<Uuid>,
}