use crate::api::files::multipart::*;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(
    init_multipart_upload,
    complete_multipart_upload,
    abort_multipart_upload
))]
pub struct S3Api;
