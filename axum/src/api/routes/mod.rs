pub mod auth_routes;
pub mod deck_routes;
pub mod learn_routes;
pub mod lesson_routes;
// pub mod preference_routes;
pub mod s3_routes;
pub mod task_routes;
pub mod user_routes;

pub use auth_routes::*;
pub use deck_routes::deck_routes;
pub use learn_routes::learn_routes;
pub use lesson_routes::lesson_routes;
pub use s3_routes::s3_routes;
pub use task_routes::task_routes;
pub use user_routes::user_routes;
