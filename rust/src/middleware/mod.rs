use actix_web::{HttpRequest, HttpResponse, web};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde_json::Value;

pub fn validate_jwt(req: &HttpRequest) -> Result<String, HttpResponse> {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let auth_header = req.headers().get("Authorization");
    
    match auth_header {
        Some(auth_value) if auth_value.to_str().unwrap_or("").starts_with("Bearer ") => {
            let token = auth_value.to_str().unwrap_or("").trim_start_matches("Bearer ");
            let validation = Validation::new(Algorithm::HS256);
            
            match decode::<Value>(token, &DecodingKey::from_secret(secret.as_bytes()), &validation) {
                Ok(_) => Ok(token.to_string()),
                Err(_) => Err(HttpResponse::Unauthorized().body("Invalid JWT token")),
            }
        },
        _ => Err(HttpResponse::Unauthorized().body("Missing JWT token")),
    }
}