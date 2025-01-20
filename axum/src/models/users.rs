use axum::http::{
    header::{self},
    HeaderValue,
};
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};
use validator::Validate;
use chrono::{DateTime, Utc};
use axum_extra::extract::cookie::{Cookie, SameSite};  

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


#[derive(Serialize, Deserialize, Debug)]
pub struct SignUpBody {
    pub id: String,
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub name: String,
    pub username: String,
    pub email: String,
    pub pass: String,
    pub role: String,
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


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthBody {
    pub access_token: String,
    pub token_type: String,
}

use std::env;

impl AuthBody {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }

    pub fn into_refresh(access_token: String) -> Response {
        let mut response = Json(AuthBody::new(access_token.to_string())).into_response();

        let cookie = build_auth_cookie("accessToken", access_token, false);

        response.headers_mut().insert(
            header::SET_COOKIE,
            HeaderValue::from_str(&cookie.to_string()).unwrap(),
        );

        response
    }

    pub fn into_response(access_token: String, refresh_token: String) -> Response {
        // Just convert the AuthBody to JSON response without Bearer header
        let mut response = Json(AuthBody::new(access_token.to_string())).into_response();

        // Set up the secure cookie for refresh token
        let cookies = [
            build_auth_cookie("refreshToken", refresh_token, true),
            build_auth_cookie("accessToken", access_token, false),
        ];

        response.headers_mut().insert(
            header::SET_COOKIE,
            HeaderValue::from_str(&cookies[0].to_string()).unwrap(),
        );

        response.headers_mut().append(
            header::SET_COOKIE,
            HeaderValue::from_str(&cookies[1].to_string()).unwrap(),
        );

        response
    }
}

fn build_auth_cookie(name: &str, value: String, is_refresh: bool) -> Cookie {

    let base_domain = env::var("APP_DOMAIN").unwrap_or_else(|_| "localhost".to_string());
    let domain = format!(".{}", base_domain);
    
    Cookie::build((name, value))
        .http_only(true)
        .secure(env::var("APP_ENV").unwrap() == "production")
        .same_site(SameSite::Lax)
        .max_age(if is_refresh {
            time::Duration::days(30)
        } else {
            time::Duration::minutes(15)
        })
        .path("/")
        .domain(domain)
        .build()
}


// Simple struct to hold the invite data
#[derive(Serialize, Deserialize)]
pub struct InviteToken {
   pub teacher_id: String,
    pub created_at: DateTime<Utc>,
}


impl InviteToken {
    pub fn new(teacher_id: String) -> Self {
        Self { teacher_id,
        created_at: Utc::now(),
        }
    }
}

#[derive(Deserialize)]
pub struct BindPayload {
   pub  student_id: String,  // Could be either the person inviting or being invited
   pub  invite_token: String,
}