use crate::api::account::*;
use utoipa::OpenApi;

use ogonek_types::DashboardData;

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
    components(schemas(ogonek_types::User, ogonek_types::InviteToken, DashboardData))
)]
pub struct UserApi;
