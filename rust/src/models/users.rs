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

use tower_cookies::Cookie;

impl User {
    pub fn into_response(self, access_token: String, refresh_token: String) -> Response {
        // Normal headers (Bearer token)
        let mut response = (
            [(
                header::AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", access_token)).unwrap(),
            )]
            .into_iter()
            .collect::<HeaderMap>(),
            Json(self),
        )
            .into_response();

        // Cookie is added via Set-Cookie header
        let cookie = Cookie::build(("refresh-token", refresh_token))
            .http_only(true)
            .secure(true)
            .path("/")
            .build();

        response.headers_mut().insert(
            header::SET_COOKIE,
            HeaderValue::from_str(&cookie.to_string()).unwrap(),
        );

        response
    }
}
