use utoipa::{
    Modify, OpenApi,
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
};

pub mod auth;
pub mod deck;
pub mod learn;
pub mod lesson;
pub mod s3;
pub mod task;
pub mod user;

pub const LESSON_TAG: &str = "Lesson";
pub const TASK_TAG: &str = "Task"; // files are here
pub const DECK_TAG: &str = "Deck";
pub const USER_TAG: &str = "User"; // preferences, profile, self, student
pub const AUTH_TAG: &str = "Auth";
pub const LEARN_TAG: &str = "Learn";

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "api_key",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("X-API-Key"))),
            )
        }
    }
}

// Main API doc that aggregates everything
#[derive(OpenApi)]
#[openapi(
    nest(
        (path = "/api/v1/lessons", api = lesson::LessonApi),
        (path = "/api/v1/tasks", api = task::TaskApi),
        (path = "/api/v1/decks", api = deck::DeckApi),
        (path = "/api/v1/users", api = user::UserApi),
        (path = "/api/v1/auth", api = auth::AuthApi),
        (path = "/api/v1/learn", api = learn::LearnApi),
        (path = "/api/v1/s3", api = s3::S3Api)
    ),
    modifiers(&SecurityAddon),
    servers(
        (url = "/", description = "Current server")
    ),
    tags(
        (name = LESSON_TAG, description = "Lesson API"),
        (name = TASK_TAG, description = "Task API"),
        (name = DECK_TAG, description = "Deck API"),
        (name = USER_TAG, description = "User API"),
        (name = AUTH_TAG, description = "Auth API"),
        (name = LEARN_TAG, description = "Learn API")
    ),
    info(
        title = "Ogonek API",
        contact(
            name = "API Support",
            email = "danila.volkov@noxlovette.com"
        )
    )
)]
pub struct ApiDoc;
