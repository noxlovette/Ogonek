use crate::api::account;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        account::fetch_me,
        account::update_user,
        account::delete_user,
        account::fetch_inviter,
        account::fetch_profile,
        account::upsert_profile,
    ),
    components(schemas(crate::models::User, crate::models::InviteToken,))
)]
pub struct UserApi;
