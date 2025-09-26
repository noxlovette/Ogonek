use crate::api::account;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        account::signup,
        account::signin,
        account::refresh,
        account::bind_student_to_teacher,
        account::generate_invite_link,
    ),
    components(schemas(
        crate::types::SignUpPayload,
        crate::types::AuthPayload,
        crate::types::TokenPair,
        crate::types::RefreshTokenPayload,
        crate::types::RefreshTokenResponse,
        crate::types::BindPayload,
        crate::types::InviteQuery,
        crate::types::InviterQuery,
    ))
)]
pub struct AuthApi;
