use crate::auth::error::PasswordHashError;
use sqlx::error::Error as SqlxError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("Database error")]
    Db,
    #[error("Not Found: {0}")]
    NotFound(String),
    #[error("Transaction failed")]
    TransactionFailed,
    #[error("Already exists")]
    AlreadyExists,
}

impl From<SqlxError> for DbError {
    fn from(error: SqlxError) -> Self {
        eprintln!("Database error: {}", error);
        Self::Db
    }
}

impl From<PasswordHashError> for DbError {
    fn from(error: PasswordHashError) -> Self {
        eprintln!("{error}");
        Self::TransactionFailed
    }
}
