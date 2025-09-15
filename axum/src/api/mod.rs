pub mod error;
pub mod handlers;
pub mod middleware;
pub mod routes;
pub use handlers::*;

// Re-export OpenAPI tags for use in handlers
pub use crate::openapi::{
    ADMIN_TAG, AUTH_TAG, CALENDAR_TAG, DECK_TAG, LEARN_TAG, LESSON_TAG, STATE_TAG, TASK_TAG, USER_TAG,
};
