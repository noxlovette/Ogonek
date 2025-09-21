use crate::api::CALENDAR_TAG;
use crate::api::error::APIError;
use crate::auth::Claims;
use crate::db::crud::core::calendar::event::delete;
use crate::schema::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;

/// Delete an event
#[utoipa::path(
    delete,
    path = "/events/{id}",
    tag = CALENDAR_TAG,
    params(
        ("id" = String, Path, description = "Event UID")
    ),
    responses(
        (status = 204, description = "Event deleted successfully"),
        (status = 404, description = "Event not found"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn delete_event(
    State(state): State<AppState>,
    Path(id): Path<String>,
    _claims: Claims,
) -> Result<StatusCode, APIError> {
    delete(&state.db, &id).await?;
    Ok(StatusCode::NO_CONTENT)
}
