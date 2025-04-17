use crate::api::core;
use crate::schema::AppState;
use axum::routing::get;
use axum::Router;

// watch out - uses lessonIDs associated with the note
pub fn notes_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(core::list_student_notes))
        .route(
            "/n/{id}",
            get(core::fetch_student_note)
                .patch(core::upsert_student_note)
                .delete(core::delete_student_notes),
        )
}
