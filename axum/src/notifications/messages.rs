// src/notifications/types.rs
use serde::{Deserialize, Serialize};

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
            Self::LessonCreated => {
                format!("You have a new Lesson")
            }
        }
    }
}

/// Escapes user content for Telegram MarkdownV2
/// ONLY escapes characters that can break parsing in user input
/// Does NOT escape markdown formatting characters like [], (), etc.
fn escape_markdown_v2(text: &str) -> String {
    // Only escape these specific chars that break MarkdownV2 in user content
    text.chars()
        .map(|c| match c {
            '_' | '*' | '[' | ']' | '~' | '`' | '>' | '#' | '+' | '=' | '|' | '{' | '}' | '.'
            | '!' | '\\' => {
                format!("\\{}", c)
            }
            _ => c.to_string(),
        })
        .collect()
}
