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
        crate::models::SignUpPayload,
        crate::models::AuthPayload,
        crate::models::TokenPair,
        crate::models::RefreshTokenPayload,
        crate::models::RefreshTokenResponse,
        crate::models::BindPayload,
        crate::models::InviteQuery,
        crate::models::InviterQuery,
    ))
)]
pub struct AuthApi;
