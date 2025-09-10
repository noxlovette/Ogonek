pub mod error;
pub mod handlers;
pub mod middleware;
pub mod routes;
pub use handlers::*;

// Re-export OpenAPI tags for use in handlers
pub use crate::openapi::{AUTH_TAG, USER_TAG, LESSON_TAG, TASK_TAG, DECK_TAG, STATE_TAG, LEARN_TAG, ADMIN_TAG};
