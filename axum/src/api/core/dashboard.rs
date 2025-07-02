use crate::api::error::APIError;
use crate::auth::Claims;
use crate::db::crud::account::{student, user};
use crate::db::crud::{account, core};
use crate::models::DashboardData;
use crate::schema::AppState;
use axum::extract::Json;
use axum::extract::State;

pub async fn fetch_dashboard(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<DashboardData>, APIError> {
    let user = user::find_by_id(&state.db, &claims.sub).await?;
    let students = student::find_all(&state.db, &claims.sub).await?;
    let tasks = core::task::fetch_recent(&state.db, &claims.sub).await?;
    let lessons = core::lesson::find_recent(&state.db, &claims.sub).await?;
    let profile =
        account::profile::find_by_id(&state.db, &claims.sub, user.role == "student").await?;

    Ok(Json(DashboardData {
        students,
        lessons,
        tasks,
        user,
        profile,
    }))
}
