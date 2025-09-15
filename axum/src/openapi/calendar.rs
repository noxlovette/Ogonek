use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api::handlers::core::calendar::fetch_calendar,
        crate::api::handlers::core::calendar::list_calendars,
        crate::api::handlers::core::calendar::create_calendar,
        crate::api::handlers::core::calendar::delete_calendar,
        crate::api::handlers::core::calendar::update_calendar,
    ),
    components(
        schemas(
            crate::types::Calendar,
            crate::types::CalendarCreate,
            crate::types::CalendarUpdate,
        )
    )
)]
pub struct CalendarApi;