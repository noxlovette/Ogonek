use crate::api::CALENDAR_TAG;
use crate::api::error::APIError;
use crate::auth::Claims;
use crate::db::crud::core::calendar::event::create;
use crate::schema::AppState;
use crate::types::EventCreate;
use axum::extract::{Json, State};
use axum::http::StatusCode;
/// Create a new event
#[utoipa::path(
    post,
    path = "/events",
    tag = CALENDAR_TAG,
    request_body = EventCreate,
    responses(
        (status = 201, description = "Event created successfully", body = String),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn create_event(
    State(state): State<AppState>,
    claims: Claims,
    Json(mut payload): Json<EventCreate>,
) -> Result<StatusCode, APIError> {
    if payload.dtend_time.is_none() {
        payload.dtend_time = Some(payload.dtstart_time + chrono::Duration::hours(1));
    }

    create(&state.db, &claims.sub, payload).await?;

    Ok(StatusCode::NO_CONTENT)
}
