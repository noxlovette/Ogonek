// src/posts/mod.rs
use crate::models::{Task, NewTask};
use crate::db::postgres::establish_connection;
use diesel::result::Error as DieselError;
use crate::schema::tasks;
use diesel::prelude::*;
use uuid::Uuid;
use ulid::Ulid;

// CREATE
pub fn create_task(
    title: &str, 
    content: &str, 
    priority: &i16, 
    completed: &bool, 
    due_date: &Option<chrono::DateTime<chrono::Utc>>, 
    file: &Option<String>, 
    assignee_id: &Uuid
) -> Result<Task, DieselError> {
    let connection = &mut establish_connection();

    let id = Ulid::new().to_string(); // Generate ULID
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


// DELETE


// RETRIEVE SINGLE


// RETRIEVE ALL
