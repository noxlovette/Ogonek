use axum::{extract::State, Json};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use surrealdb::engine::remote::ws::Client;
use surrealdb::{sql::Thing, Surreal};
use tokio::sync::Mutex;

use crate::db::users::{get_new_token, make_new_user};

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Response {
    message: String,
    persons: Vec<Person>,
}

pub async fn create_user_endpoint(
    State(db): State<Arc<Mutex<Surreal<Client>>>>,
) -> Result<Json<Response>, (StatusCode, String)> {
    let mut db = db.lock().await;

    db.query("CREATE person SET name = 'Created by root'")
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let user = make_new_user(&mut db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    get_new_token(&mut db, &user)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    db.query("CREATE person SET name = 'Created by record user'")
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let persons = db
        .select::<Vec<Person>>("person")
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    db.query("DELETE person")
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let remaining_persons = db
        .select::<Vec<Person>>("person")
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(Response {
        message: "User operations completed".to_string(),
        persons: remaining_persons,
    }))
}
