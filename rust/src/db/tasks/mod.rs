// src/posts/mod.rs
use crate::db::postgres::establish_connection;
use crate::models::{NewTask, Task, UpdateTask};
use crate::schema::tasks;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use ulid::Ulid;
use uuid::Uuid;

// CREATE
pub fn create_task(
    title: &str,
    content: &str,
    priority: &i16,
    completed: &bool,
    due_date: &Option<chrono::DateTime<chrono::Utc>>,
    file: &Option<String>,
    assignee_id: &Uuid,
) -> Result<Task, DieselError> {
    let connection = &mut establish_connection();

    let id = Ulid::new().to_string();
    let new_task = NewTask {
        id,
        title,
        content,
        priority,
        completed,
        due_date: due_date.as_ref(),
        file: file.as_deref(),
        assignee_id,
    };

    diesel::insert_into(tasks::table)
        .values(&new_task)
        .get_result(connection)
}

// UPDATE
pub fn update_task(
    id: &str,
    title: Option<&str>,
    content: Option<&str>,
    priority: Option<&i16>,
    completed: Option<&bool>,
    due_date: Option<&chrono::DateTime<chrono::Utc>>,
    file: Option<&str>,
    assignee_id: Option<&Uuid>,
) -> Result<Task, DieselError> {
    let connection = &mut establish_connection();

    let update_task = UpdateTask {
        title: title.map(|s| s.to_string()),
        content: content.map(|s| s.to_string()),
        priority: priority.copied(),
        completed: completed.copied(),
        due_date: due_date.cloned(),
        file: file.map(|s| s.to_string()),
        assignee_id: assignee_id.copied(),
    };

    diesel::update(tasks::table.find(id))
        .set(&update_task)
        .get_result(connection)
}

// DELETE

// RETRIEVE SINGLE

// RETRIEVE ALL
