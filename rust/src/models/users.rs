use axum::http::{
    header::{self, HeaderMap},
    HeaderValue,
};
use axum::response::{IntoResponse, Response};
use axum::Json;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserBody {
    name: String,
    username: String,
    email: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct SignUpPayload {
    pub name: String,
    pub email: String,
    pub pass: String,
    pub username: String,
    pub role: String,
}



#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Option<RecordId>,
    pub name: String,
    pub username: String,
    pub email: String,
    pub joined_at: Option<DateTime<Utc>>,
    pub role: Option<String>,
    pub pass: Option<String>,
}

impl User {
    pub fn into_response(self, token: String) -> Response {
        let mut headers = HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token)).expect("Invalid Token Format"),
        );

        (headers, Json(self)).into_response()
    }
}

#[derive(Debug, Deserialize)]
pub struct AuthPayload {
    pub username: String,
    pub pass: String,
}
