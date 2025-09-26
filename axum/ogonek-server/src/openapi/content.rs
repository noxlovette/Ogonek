use crate::api::content::*;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(fetch_content_public))]
pub struct ContentApi;
