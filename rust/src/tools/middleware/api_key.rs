use axum::{
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::Response,
    routing::Router,
};

#[derive(Clone, Debug)]
struct ApiKey(String);

#[derive(Debug)]
enum ApiKeyError {
    Missing,
    Invalid,
}

async fn validate_api_key<B>(
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let api_key = request
    .headers()
    .get("X-API-Key")
    .ok_or(ApiKeyError::Missing)?
    .to_str()
    .map_err(|_| ApiKeyError::Invalid)?;
    
    ;
}