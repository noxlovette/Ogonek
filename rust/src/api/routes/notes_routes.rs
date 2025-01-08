use crate::api::notes;
use crate::db::init::AppState;
use axum::routing::get;
use axum::Router;

// watch out - uses lessonIDs associated with the note
pub fn notes_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(notes::list_student_notes))
        .route(
            "/n/{id}",
            get(notes::fetch_student_note)
                .patch(notes::upsert_student_note)
                .delete(notes::delete_student_notes),
        )
}
