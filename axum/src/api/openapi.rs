use super::core::{deck, learn, lesson, task};
use utoipa::{
    Modify, OpenApi,
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
};
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
        lesson::fetch_lesson,
        lesson::create_lesson,
        lesson::delete_lesson,
        lesson::update_lesson,
        task::fetch_task,
        task::list_tasks,
        task::create_task,
        task::delete_task,
        task::toggle_task,
        task::update_task,
        deck::create_deck,
        deck::fetch_deck,
        deck::fetch_deck_list,
        deck::fetch_deck_list_public,
        deck::update_deck,
        deck::delete_deck,
        learn::subscribe_to_deck,
        learn::unsubscribe_from_deck

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
