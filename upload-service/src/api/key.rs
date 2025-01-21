use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use dotenvy::dotenv;

#[derive(Clone, Debug)]
struct ApiKey(String);

impl ApiKey {
    fn from_header(value: &str) -> Option<Self> {
        // Prevent empty strings
        if value.trim().is_empty() {
            return None;
        }
        Some(ApiKey(value.to_string()))
    }

    fn as_str(&self) -> &str {
        &self.0
    }
}

fn get_key(headers: &HeaderMap) -> Option<ApiKey> {
    headers
        .get("x-api-key")
        .and_then(|value| value.to_str().ok())
        .and_then(ApiKey::from_header)
}

fn key_valid(api_key: &ApiKey) -> bool {
    dotenv().ok();
    let valid_key = std::env::var("API_KEY").expect("API_KEY must be set");
    api_key.as_str() == valid_key
}

pub async fn validate_api_key(request: Request, next: Next) -> Result<Response, StatusCode> {
    let api_key = get_key(request.headers()).ok_or(StatusCode::UNAUTHORIZED)?;

    // Here you can add your key validation logic
    if !key_valid(&api_key) {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(next.run(request).await)
}
