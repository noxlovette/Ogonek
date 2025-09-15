use crate::api::core::{self, list_events_day, state};
use crate::schema::AppState;
use axum::Router;
use axum::routing::{get, post, put};

pub fn lesson_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(core::list_lessons).post(core::create_lesson))
        .route(
            "/{id}",
            get(core::fetch_lesson)
                .patch(core::update_lesson)
                .delete(core::delete_lesson),
        )
        .route(
            "/{id}/photo",
            post(core::upsert_photo).delete(core::delete_photo),
        )
}

pub fn task_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(core::list_tasks).post(core::create_task))
        .route(
            "/{id}",
            get(core::fetch_task)
                .patch(core::update_task)
                .delete(core::delete_task)
                .put(core::toggle_task),
        )
}
use crate::api::core::{deck, learn};

pub fn deck_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(deck::list_decks).post(deck::create_deck))
        .route(
            "/{id}",
            get(deck::fetch_deck)
                .patch(deck::update_deck)
                .delete(deck::delete_deck)
                .post(learn::reset_deck_progress),
        )
        .route("/{id}/duplicate", post(deck::duplicate_deck))
        .route("/public", get(deck::list_decks_public))
}
pub fn learn_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/subscribe/{id}",
            post(learn::subscribe_to_deck).delete(learn::unsubscribe_from_deck),
        )
        .route("/{id}", put(learn::update_card_progress))
        .route("/", get(learn::fetch_due_cards))
}

pub fn state_routes() -> Router<AppState> {
    Router::new()
        .route("/dashboard", get(state::fetch_dashboard))
        .route("/badges", get(state::fetch_badges))
        .route("/context", get(state::fetch_context))
}

pub fn calendar_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(core::list_calendars).post(core::create_calendar))
        .route(
            "/{id}",
            get(core::fetch_calendar)
                .patch(core::update_calendar)
                .delete(core::delete_calendar),
        )
        .route(
            "/{calendar_id}/events",
            get(core::list_events).post(core::create_event),
        )
        .route("/{calendar_id}/events/{day}", get(list_events_day))
        .route(
            "/events/{id}",
            get(core::fetch_event)
                .patch(core::update_event)
                .delete(core::delete_event),
        )
        .route(
            "/events/{event_id}/attendees",
            get(core::list_attendees).post(core::create_attendee),
        )
        .route(
            "/attendees/{id}",
            get(core::fetch_attendee)
                .patch(core::update_attendee)
                .delete(core::delete_attendee),
        )
}
