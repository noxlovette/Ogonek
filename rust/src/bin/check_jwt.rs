// src/bin/check_jwt.rs
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde_json::Value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // The JWT token you want to verify
    let token = std::env::var("JET_TOKEN").expect("JET_TOKEN must be set");

    // Since we're just checking the format, we'll use a dummy secret for validation
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    // Prepare validation structure
    let validation = Validation::new(Algorithm::HS256);

    match decode::<Value>(token, &DecodingKey::from_secret(secret.as_bytes()), &validation) {
        Ok(token_data) => {
            println!("JWT Token is valid!");
            println!("Claims: {:?}", token_data.claims);
        },
        Err(e) => {
            println!("JWT Token is invalid or not in the correct format: {}", e);
        }
    }

    Ok(())
}