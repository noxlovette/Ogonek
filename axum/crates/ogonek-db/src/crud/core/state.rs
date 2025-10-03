use ogonek_types::AppContext;
use sqlx::PgPool;

use crate::{
    DbError,
    core::account::{
        preferences,
        profile::{read_by_id, read_call_url_for_student},
        student, user,
    },
};

pub async fn read_context(db: &PgPool, user_id: &str) -> Result<AppContext, DbError> {
    let mut tx = db.begin().await?;

    let preferences = preferences::read_or_create_defaults(&mut *tx, &user_id).await?;
    let user = user::read_by_id(&mut *tx, &user_id).await?;
    let students = student::read_all(&mut *tx, &user_id).await?;
    let profile = read_by_id(&mut *tx, &user_id).await?;
    let call_url = read_call_url_for_student(&mut *tx, &user_id).await?;

    tx.commit().await?;
    Ok(AppContext {
        user,
        profile,
        students,
        preferences,
        call_url,
    })
}
