// src/auth/mod.rs
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Header, Algorithm, EncodingKey};
use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String, // Subject, typically the user identifier
    exp: usize,  // Expiration time (as UTC timestamp)
    is_superuser: bool,
    username: String,
    // Add more claims as needed
}

impl Claims {
    pub fn with_details(id: Uuid, is_superuser: bool, username: &str) -> Self {
        Claims {
            sub: id.to_string(),
            exp: (Utc::now() + Duration::hours(1)).timestamp() as usize, // Token expires in 1 hour
            is_superuser,
            username: username.to_string(),
        }
    }
}

pub fn generate_jwt(user_id: Uuid, is_superuser: bool, username: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let claims = Claims::with_details(user_id, is_superuser, username);
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    encode(&Header::new(Algorithm::HS256), &claims, &EncodingKey::from_secret(secret.as_ref()))
}