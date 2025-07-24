use super::{
    account,
    core::{deck, learn, lesson, task},
};
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
        lesson::update_lesson,
        lesson::delete_lesson,

        task::list_tasks,
        task::fetch_task,
        task::create_task,
        task::toggle_task,
        task::update_task,
        task::delete_task,

        deck::fetch_deck_list,
        deck::fetch_deck_list_public,
        deck::fetch_deck,
        deck::create_deck,
        deck::update_deck,
        deck::delete_deck,

        learn::fetch_due_cards,
        learn::update_card_progress,
        learn::reset_deck_progress,
        learn::subscribe_to_deck,
        learn::unsubscribe_from_deck,

        account::fetch_me,
        account::update_user,
        account::delete_user,
        account::fetch_inviter,
        account::signup,
        account::signin,
        account::refresh,
        account::bind_student_to_teacher,
        account::generate_invite_link,

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
