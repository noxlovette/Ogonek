use crate::api::content::*;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(fetch_content))]
pub struct ContentApi;
