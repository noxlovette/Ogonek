use crate::helpers::RRuleError;
use sqlx::error::Error as SqlxError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("Database error: {0}")]
    Database(SqlxError),
    #[error("Not Found: {0}")]
    NotFound(String),
    #[error("Transaction failed")]
    TransactionFailed,
    #[error("Already exists: {0}")]
    AlreadyExists(String),
    #[error("The event is not recurring")]
    NotRecurring,
    #[error("Invalid recurrence id")]
    InvalidRecurrenceId,
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("Invalid recurrence rule: {0}")]
    InvalidRRule(#[from] RRuleError),
}

impl From<SqlxError> for DbError {
    fn from(error: SqlxError) -> Self {
        match error {
            SqlxError::RowNotFound => Self::NotFound("Resource not found".to_string()),
            other => {
                eprintln!("Database error: {other}");
                Self::Database(other)
            }
        }
    }
}
