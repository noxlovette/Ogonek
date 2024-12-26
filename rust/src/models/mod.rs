// models/lib.rs
pub mod users;
pub mod lessons;

pub use users::{User, NewUser, UserUpdate};
pub use lessons::Lesson;