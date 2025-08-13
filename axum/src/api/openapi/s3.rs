use crate::api::files::*;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(
    init_multipart_upload,
    complete_multipart_upload,
    abort_multipart_upload,
    fetch_presigned_url,
    fetch_presigned_urls_batch,
    delete_file,
))]
pub struct S3Api;
