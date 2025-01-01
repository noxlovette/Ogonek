use axum::extract::Json;
use axum::extract::State;
use axum::Error;
use serde::{Deserialize, Serialize};
use surrealdb::error::Db;
use crate::auth::Token;
use crate::db::error::Error as DbError;
use crate::db::AppState;
use crate::auth::AuthError;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserBody {
    name: String,
    username: String,
    email: String,
}

impl UserBody {
    fn new(name: String, username: String, email: String) -> Self {
        Self {
            name,
            username,
            email,
        }
    }
}

pub async fn fetch_user(
    State(state): State<AppState>,
    token: Token,
) -> Result<Json<Option<UserBody>>, DbError> {
    
    
    tracing::info!("Attempting fetch for user");

    let db = &state.db;
    db.authenticate(token.as_str()).await?;

    tracing::info!("Token if it worked: {:?}", token.as_str());

    // The query returns a Vec<UserBody> since SurrealDB always returns an array
    let result: Vec<UserBody> = db
        .query("SELECT name, email, username FROM user WHERE id = $auth.id")
        .await?
        .take(0)?; // take(0) gets the first result set

    // Get the first user if any exists
    let user = result.into_iter().next();
    
    Ok(Json(user))
}