use crate::auth::error::PasswordHashError;
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
}

impl From<SqlxError> for DbError {
    fn from(error: SqlxError) -> Self {
        eprintln!("Database error: {error}");
        Self::Database(error)
    }
}

impl From<PasswordHashError> for DbError {
    fn from(error: PasswordHashError) -> Self {
        eprintln!("{error}");
        Self::TransactionFailed
    }
}
