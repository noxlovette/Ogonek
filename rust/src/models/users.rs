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

#[derive(Serialize, Deserialize)]
pub struct Credentials<'a> {
    pub username: &'a str,
    pub pass: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignUpPayload {
    pub name: String,
    pub email: String,
    pub pass: String,
    pub username: String,
    pub role: String,
}

#[derive(Serialize, Deserialize)]
pub struct SignUpCredentials<'a> {
    pub name: &'a str,
    pub pass: &'a str,
    pub email: &'a str,
    pub username: &'a str,
    pub role: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Option<RecordId>,
    pub name: String,
    pub username: String,
    pub email: String,
    pub joined_at: Option<DateTime<Utc>>,
    pub role: Option<String>,
    // #[serde(skip_deserializing)] // This field will be ignored during deserialization
    // #[serde(skip_serializing_if = "Option::is_none")]
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
