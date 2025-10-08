use crate::api::{content::*, core::task};

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(fetch_content_public, task::fetch_task_public),
    components(schemas(ogonek_types::TaskPublic,))
)]
pub struct ContentApi;
