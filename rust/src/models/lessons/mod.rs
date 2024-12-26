use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Insertable)]
#[diesel(table_name = crate::schema::lessons)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Lesson {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub category: String,
    pub topic: String,
    pub manual_date: Option<chrono::DateTime<chrono::Utc>>,
    pub bookmarked: bool,
    pub assignee_id: Uuid,
}