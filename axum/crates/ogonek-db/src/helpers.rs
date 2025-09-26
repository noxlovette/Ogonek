use crate::{db::error::DbError, types::users::User};

pub trait FromQuery: Sized {
    fn from_query_result(result: Vec<Self>) -> Result<Self, DbError> {
        result.into_iter().next().ok_or(DbError::TransactionFailed)
    }
}

impl FromQuery for User {}
