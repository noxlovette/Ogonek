use crate::api::account::*;
use crate::api::core::dashboard;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        fetch_me,
        update_user,
        delete_user,
        fetch_inviter,
        fetch_profile,
        upsert_profile,
        upsert_student,
        remove_student,
        update_student,
        fetch_student,
        list_students,
        dashboard::fetch_dashboard,
    ),
    components(schemas(crate::models::User, crate::models::InviteToken,))
)]
pub struct UserApi;
