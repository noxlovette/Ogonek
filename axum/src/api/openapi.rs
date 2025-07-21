use super::core::lesson;
use utoipa::{
    Modify, OpenApi,
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
};
pub const LESSON_TAG: &str = "lesson";
const TASK_TAG: &str = "task"; // files are here
const DECK_TAG: &str = "deck";
const USER_TAG: &str = "user"; // preferences, profile, self, student
const AUTH_TAG: &str = "auth";

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "api_key",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("todo_apikey"))),
            )
        }
    }
}

#[derive(OpenApi)]
#[openapi(
    modifiers(&SecurityAddon),
    paths(
        lesson::list_lessons,
        lesson::fetch_lesson
    ),
    tags(
        (name = LESSON_TAG, description = "Lesson API endpoints"),
        (name = TASK_TAG, description = "Task API endpoints"),
        (name = DECK_TAG, description = "Deck API endpoints"),
        (name = USER_TAG, description = "User API endpoints"),
        (name = AUTH_TAG, description = "Auth API endpoints")
    )
)]
pub struct ApiDoc;

/// Get health of the API.
#[utoipa::path(
    method(get, head),
    path = "/api/health",
    responses(
        (status = OK, description = "Success", body = str, content_type = "text/plain")
    )
)]
pub async fn health() -> &'static str {
    "ok"
}
