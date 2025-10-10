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
        account::resend_verification,
        account::confirm_email
    ),
    components(schemas(
        ogonek_types::SignUpPayload,
        ogonek_types::AuthPayload,
        ogonek_types::TokenPair,
        ogonek_types::RefreshTokenPayload,
        ogonek_types::RefreshTokenResponse,
        ogonek_types::BindPayload,
        ogonek_types::InviteQuery,
        ogonek_types::InviterQuery,
        ogonek_types::EmailVerificationQuery,
    ))
)]
pub struct AuthApi;
