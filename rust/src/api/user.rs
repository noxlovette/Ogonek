use crate::db::auth::{get_new_token, make_new_user, Person};
use crate::db::AppState;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;

pub async fn test_user_endpoint(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let db = &state.db;

    db.query("CREATE person SET name = 'Created by root'")
        .await
        .unwrap();

    let user = make_new_user(&db).await.unwrap();

    get_new_token(&db, &user).await.unwrap();

    db.query("CREATE person SET name = 'Created by record user'")
        .await
        .unwrap();

    println!(
        "Two `person` records: {:?}\n",
        db.select::<Vec<Person>>("person").await
    );

    db.query("DELETE person").await.unwrap();

    println!(
        "`person` created by root is still there: {:?}\n",
        db.select::<Vec<Person>>("person").await
    );
    tracing::info!("User endpoint test passed");

    Ok(StatusCode::OK)
}
