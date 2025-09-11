use utoipa::OpenApi;

pub mod admin;
pub mod auth;
pub mod content;
pub mod deck;
pub mod files;
pub mod learn;
pub mod lesson;
pub mod notifications;
pub mod state;
pub mod task;
pub mod user;
pub const LESSON_TAG: &str = "Lesson";
pub const TASK_TAG: &str = "Task"; // files are here
pub const DECK_TAG: &str = "Deck";
pub const USER_TAG: &str = "User"; // profile, self, student
pub const STATE_TAG: &str = "State"; // preferences, badges, dashboard
pub const AUTH_TAG: &str = "Auth";
pub const LEARN_TAG: &str = "Learn";

pub const ADMIN_TAG: &str = "Admin";
pub const CONTENT_TAG: &str = "Content";

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
        (path = "/api/v1/files", api = files::FilesApi),
        (path = "/api/v1/notifications", api = notifications::NotificationApi),
        (path = "/api/v1/state", api = state::StateApi),
        (path = "/api/v1/content", api = content::ContentApi),
        (path = "/api/v1/admin", api = admin::AdminApi)

    ),
    servers(
        (url = "https://api.ogonek.app", description = "Production server"),
        (url = "https://api.staging.ogonek.app", description = "Staging server"),
        (url = "http://localhost:3000", description = "Local development server")
    ),
    tags(
        (name = LESSON_TAG,description = "Lesson API"),
        (name = TASK_TAG,description = "Task API"),
        (name = DECK_TAG,description = "Deck API"),
        (name = USER_TAG,description = "User API"),
        (name = AUTH_TAG,description = "Auth API"),
        (name = LEARN_TAG,description = "Learn API"),
        (name = STATE_TAG,description = "State API"),
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
