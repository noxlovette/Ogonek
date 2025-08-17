use serde::{Deserialize, Serialize};

use crate::types::NotificationPayload;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationType {
    #[serde(rename = "teacherNotify")]
    TeacherNotify { username: String },
    #[serde(rename = "completed")]
    Completed {
        task: String,
        username: String,
        id: String,
    },
    #[serde(rename = "reminder")]
    Reminder { task: String, due_date: String },
    #[serde(rename = "deckCreated")]
    DeckCreated { title: String, id: String },
    #[serde(rename = "taskCreated")]
    TaskCreated {
        title: String,
        id: String,
        date: String,
    },
    #[serde(rename = "lessonCreated")]
    LessonCreated,
}

impl NotificationType {
    pub fn to_message(&self) -> String {
        match self {
            Self::TeacherNotify { username } => {
                format!(
                    "{} needs homework\\. Add more on [Ogonek](https://ogonek\\.app/t/tasks)",
                    escape_markdown_v2(username)
                )
            }
            Self::Completed { task, username, id } => {
                format!(
                    "{} for {} has been completed\\. View the result on [Ogonek](https://ogonek\\.app/t/tasks/{})",
                    escape_markdown_v2(task),
                    escape_markdown_v2(username),
                    id
                )
            }
            Self::Reminder { task, due_date } => {
                format!(
                    "Don't forget to complete \"{}\" by {}",
                    escape_markdown_v2(task),
                    escape_markdown_v2(due_date)
                )
            }
            Self::DeckCreated { title, id } => {
                format!(
                    "A new deck has been added: \"{}\"\\. View it on [Ogonek](https://ogonek\\.app/s/flashcards/{})",
                    escape_markdown_v2(title),
                    id
                )
            }
            Self::TaskCreated { title, id, date } => {
                format!(
                    "A new task has been added: \"{}\"\\. Due Date: {}\\. View it on [Ogonek](https://ogonek\\.app/s/tasks/{})",
                    escape_markdown_v2(title),
                    escape_markdown_v2(date),
                    id
                )
            }
            Self::LessonCreated => "You have a new lesson".to_string(),
        }
    }

    pub fn to_apns_payload(&self) -> NotificationPayload {
        match self {
            Self::TeacherNotify { username } => NotificationPayload {
                title: "Homework Request".to_string(),
                body: format!("{} needs homework assigned", username),
                badge: Some(1),
                sound: Some("default".to_string()),
                data: Some(serde_json::json!({
                    "type": "teacher_notify",
                    "username": username
                })),
            },
            Self::Completed { task, username, id } => NotificationPayload {
                title: "Task Completed".to_string(),
                body: format!("{} completed: {}", username, task),
                badge: Some(1),
                sound: Some("default".to_string()),
                data: Some(serde_json::json!({
                    "type": "task_completed",
                    "task_id": id,
                    "username": username
                })),
            },
            Self::Reminder { task, due_date } => NotificationPayload {
                title: "Task Reminder".to_string(),
                body: format!("Don't forget: {} (due {})", task, due_date),
                badge: Some(1),
                sound: Some("default".to_string()),
                data: Some(serde_json::json!({
                    "type": "reminder",
                    "task": task,
                    "due_date": due_date
                })),
            },
            Self::DeckCreated { title, id } => NotificationPayload {
                title: "New Deck Available".to_string(),
                body: format!("Check out the new deck: {}", title),
                badge: Some(1),
                sound: Some("default".to_string()),
                data: Some(serde_json::json!({
                    "type": "deck_created",
                    "deck_id": id,
                    "title": title
                })),
            },
            Self::TaskCreated { title, id, date } => NotificationPayload {
                title: "New Task Assigned".to_string(),
                body: format!("{} (due: {})", title, date),
                badge: Some(1),
                sound: Some("default".to_string()),
                data: Some(serde_json::json!({
                    "type": "task_created",
                    "task_id": id,
                    "title": title,
                    "due_date": date
                })),
            },
            Self::LessonCreated => NotificationPayload {
                title: "New Lesson".to_string(),
                body: "You have a new lesson available".to_string(),
                badge: Some(1),
                sound: Some("default".to_string()),
                data: Some(serde_json::json!({
                    "type": "lesson_created"
                })),
            },
        }
    }
}

/// Escapes user content for Telegram MarkdownV2
fn escape_markdown_v2(text: &str) -> String {
    text.chars()
        .map(|c| match c {
            '_' | '*' | '[' | ']' | '(' | ')' | '~' | '`' | '>' | '#' | '+' | '-' | '=' | '|'
            | '{' | '}' | '.' | '!' | '\\' => format!("\\{c}"),
            _ => c.to_string(),
        })
        .collect()
}
