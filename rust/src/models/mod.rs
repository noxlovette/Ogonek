// models/lib.rs
pub mod users;
pub mod lessons;

pub use users::{User, NewUser};
pub use lessons::Lesson;