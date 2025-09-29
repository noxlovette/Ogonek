use crate::api::admin::content::*;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(
    fetch_content,
    list_content,
    create_content,
    delete_content,
    update_content,
    publish_content,
    unpublish_content
))]
pub struct AdminContentApi;

#[derive(OpenApi)]
#[openapi(
   nest(
   (path = "/content", api = AdminContentApi)
)
)]
pub struct AdminApi;
