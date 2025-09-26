use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use ogonek_types::NotificationPayload;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationType {
    #[serde(rename = "teacherNotify")]
    TeacherNotify { username: String },
    #[serde(rename = "completed")]
    Completed {
        task_title: String,
        username: String,
        task_id: String,
    },
    #[serde(rename = "reminder")]
    Reminder {
        task_title: String,
        due_date: String,
    },
    #[serde(rename = "deckCreated")]
    DeckCreated { deck_title: String, deck_id: String },
    #[serde(rename = "taskCreated")]
    TaskCreated {
        task_title: String,
        task_id: String,
        due_date: Option<DateTime<Utc>>,
    },
    #[serde(rename = "lessonCreated")]
    LessonCreated {
        lesson_topic: String,
        lesson_id: String,
    },
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
            Self::Completed {
                task_title,
                username,
                task_id,
            } => {
                format!(
                    "{} for {} has been completed\\. View the result on [Ogonek](https://ogonek\\.app/t/tasks/{})",
                    escape_markdown_v2(task_title),
                    escape_markdown_v2(username),
                    task_id
                )
            }
            Self::Reminder {
                task_title,
                due_date,
            } => {
                format!(
                    "Don't forget to complete \"{}\" by {}",
                    escape_markdown_v2(task_title),
                    escape_markdown_v2(due_date)
                )
            }
            Self::DeckCreated {
                deck_title,
                deck_id,
            } => {
                format!(
                    "A new deck has been added: \"{}\"\\. View it on [Ogonek](https://ogonek\\.app/s/flashcards/{})",
                    escape_markdown_v2(deck_title),
                    deck_id
                )
            }
            Self::TaskCreated {
                task_title,
                task_id,
                due_date,
            } => {
                let formatted_date = due_date
                    .map(|dt| dt.format("%B %d, %Y").to_string())
                    .unwrap_or_else(|| "no due date".to_string());

                format!(
                    "A new task has been added: \"{}\"\\. Due Date: {}\\. View it on [Ogonek](https://ogonek\\.app/s/tasks/{})",
                    escape_markdown_v2(task_title),
                    escape_markdown_v2(&formatted_date),
                    task_id
                )
            }

            Self::LessonCreated {
                lesson_id,
                lesson_topic,
            } => format!(
                "A new lesson has been added: \"{}\"\\. View it on [Ogonek](https://ogonek\\.app/s/lessons/{})",
                escape_markdown_v2(lesson_topic),
                lesson_id
            ),
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
            Self::Completed {
                task_title,
                username,
                task_id,
            } => NotificationPayload {
                title: "Task Completed".to_string(),
                body: format!("{} completed: {}", username, task_title),
                badge: Some(1),
                sound: Some("default".to_string()),
                data: Some(serde_json::json!({
                    "type": "task_completed",
                    "task_id": task_id,
                    "username": username
                })),
            },
            Self::Reminder {
                task_title,
                due_date,
            } => NotificationPayload {
                title: "Task Reminder".to_string(),
                body: format!("Don't forget: {} (due {})", task_title, due_date),
                badge: Some(1),
                sound: Some("default".to_string()),
                data: Some(serde_json::json!({
                    "type": "reminder",
                    "task_title": task_title,
                    "due_date": due_date
                })),
            },
            Self::DeckCreated {
                deck_title,
                deck_id,
            } => NotificationPayload {
                title: "New Deck Available".to_string(),
                body: format!("Check out the new deck: {}", deck_title),
                badge: Some(1),
                sound: Some("default".to_string()),
                data: Some(serde_json::json!({
                    "type": "deck_created",
                    "deck_id": deck_id,
                    "deck_title": deck_title
                })),
            },
            Self::TaskCreated {
                task_title,
                task_id,
                due_date,
            } => {
                let formatted_date = due_date
                    .map(|dt| dt.format("%B %d, %Y").to_string())
                    .unwrap_or_else(|| "no due date".to_string());

                NotificationPayload {
                    title: "New Task Assigned".to_string(),
                    body: format!("{} (due: {})", task_title, formatted_date),
                    badge: Some(1),
                    sound: Some("default".to_string()),
                    data: Some(serde_json::json!({
                        "type": "task_created",
                        "task_id": task_id,
                        "title": task_title,
                        "due_date": due_date.map(|dt| dt.to_rfc3339())
                    })),
                }
            }
            Self::LessonCreated {
                lesson_topic,
                lesson_id,
            } => NotificationPayload {
                title: "New Lesson".to_string(),
                body: format!("New lesson: {}", lesson_topic),
                badge: Some(1),
                sound: Some("default".to_string()),
                data: Some(serde_json::json!({
                    "type": "lesson_created",
                    "lesson_id": lesson_id
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
