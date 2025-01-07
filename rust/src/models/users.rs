use axum::http::{
    header::{self, HeaderMap},
    HeaderValue,
};
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;
use validator::Validate;

#[derive(Deserialize, Debug)]
pub struct UserBody {
    name: String,
    username: String,
    email: String,
}

#[derive(Serialize, Deserialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SignUpPayload {
    #[validate(length(min = 3, max = 16))]
    pub name: String,

    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8, max = 32))]
    pub pass: String,

    #[validate(length(min = 3, max = 16))]
    pub username: String,
    pub role: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserUpdate {
    pub name: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub pass: Option<String>,
    pub role: Option<String>,
    pub verified: Option<bool>,
}

#[serde_with::serde_as]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub name: String,
    pub username: String,
    pub email: String,
    pub pass: String,
    pub role: String,
    #[serde_as(as = "Rfc3339")]
    pub joined: OffsetDateTime,
    pub verified: bool,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AuthPayload {
    #[validate(length(min = 3, max = 16))]
    pub username: String,
    #[validate(length(min = 8, max = 32))]
    pub pass: String,
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