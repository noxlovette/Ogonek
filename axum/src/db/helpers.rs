use crate::db::error::DbError;
use crate::models::users::User;

pub trait FromQuery: Sized {
    fn from_query_result(result: Vec<Self>) -> Result<Self, DbError> {
        result.into_iter().next().ok_or(DbError::NotFound) // or whatever error type you prefer
    }
}

impl FromQuery for User {}
