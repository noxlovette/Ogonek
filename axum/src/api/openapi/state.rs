use crate::api::core::state::*;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(fetch_dashboard, fetch_context, fetch_badges))]
pub struct StateApi;
