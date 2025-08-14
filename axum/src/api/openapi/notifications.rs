use crate::api::notifications::*;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(register_device_token, request_hw))]
pub struct NotificationApi;
