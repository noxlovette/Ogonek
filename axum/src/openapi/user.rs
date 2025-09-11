use crate::api::account::*;
use crate::types::DashboardData;
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
    ),
    components(schemas(crate::types::User, crate::types::InviteToken, DashboardData))
)]
pub struct UserApi;
