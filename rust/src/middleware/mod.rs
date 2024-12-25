use actix_web::{HttpRequest, HttpResponse};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde_json::Value;

pub fn validate_jwt(req: &HttpRequest) -> Result<Value, HttpResponse> {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let auth_header = req.headers().get("Authorization");
    
    match auth_header {
        Some(auth_value) if auth_value.to_str().unwrap_or("").starts_with("Bearer ") => {
            let token = auth_value.to_str().unwrap_or("").trim_start_matches("Bearer ");
            let validation = Validation::new(Algorithm::HS256);
            
            match decode::<Value>(token, &DecodingKey::from_secret(secret.as_bytes()), &validation) {
                Ok(token_data) => Ok(token_data.claims),
                Err(_) => Err(HttpResponse::Unauthorized().body("Invalid JWT token")),
            }
        },
        _ => Err(HttpResponse::Unauthorized().body("Missing JWT token")),
    }
}

pub fn check_superuser_and_noxlovette(claims: &Value) -> Result<(), HttpResponse> {
    if let Some(claims_obj) = claims.as_object() {
        if claims_obj.get("username").and_then(|v| v.as_str()) == Some("noxlovette") &&
           claims_obj.get("is_superuser").and_then(|v| v.as_bool()) == Some(true) {
            Ok(())
        } else {
            Err(HttpResponse::Forbidden().body("Insufficient permissions"))
        }
    } else {
        Err(HttpResponse::Unauthorized().body("Invalid token structure"))
    }
}

pub fn check_sudo(req: &HttpRequest) -> Result<(), HttpResponse> {
    match validate_jwt(req) {
        Ok(claims) => check_superuser_and_noxlovette(&claims),
        Err(e) => Err(e),
    }
}