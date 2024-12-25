use actix_web::{web, Responder, HttpResponse, HttpRequest, post};
use crate::auth::*;
use rust::api::fetch_user;
use serde::{Deserialize, Serialize};
use rust::db::users::create_user;
use jsonwebtoken::{decode, Validation};
use rust::middleware::validate_jwt;
use serde_json::Value;



#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}


#[derive(Deserialize)]
pub struct UserCreationRequest {
    username: String,
    password: String,
    confirm_password: String,
    is_superuser: bool,
}

#[derive(Serialize)]
pub struct UserCreationResponse {
    username: String,
    password: String,
    confirm_password: String,
    id: String,
}

#[post("/login")]
pub async fn login_endpoint(login_request: web::Json<LoginRequest>) -> impl Responder {
    match fetch_user(&login_request.username, &login_request.password) {
        Ok(user) => {
            match generate_jwt(user.id, user.is_superuser, &user.username) {
                Ok(token) => HttpResponse::Ok().json(web::Json(serde_json::json!({"token": token}))),
                Err(e) => {
                    eprintln!("Failed to generate JWT: {:?}", e);
                    HttpResponse::InternalServerError().body("Failed to generate token")
                },
            }
        },
        Err(e) => {
            eprintln!("Login failed: {}", e);
            HttpResponse::Unauthorized().body("Invalid username or password")
        },
    }
}


#[post("/create_user")]
pub async fn create_user_endpoint(req: HttpRequest, user_request: web::Json<UserCreationRequest>) -> impl Responder {
    // Validate JWT token
    match validate_jwt(&req) {
        Ok(token) => {
            // Decode the token to check if the user is 'noxlovette' and a superuser
            let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
            let validation = Validation::new(jsonwebtoken::Algorithm::HS256);
            match decode::<Value>(&token, &jsonwebtoken::DecodingKey::from_secret(secret.as_bytes()), &validation) {
                Ok(token_data) => {
                    if let Some(claims) = token_data.claims.as_object() {
                        if claims.get("username").and_then(|v| v.as_str()) == Some("noxlovette") &&
                           claims.get("is_superuser").and_then(|v| v.as_bool()) == Some(true) {
                            // Check if passwords match before proceeding
                            if user_request.password != user_request.confirm_password {
                                return HttpResponse::BadRequest().body("Passwords do not match");
                            }

                            match create_user(&user_request.username, &user_request.password, &user_request.is_superuser) {
                                Ok(user) => {
                                    HttpResponse::Ok().json(web::Json(UserCreationResponse {
                                        username: user_request.username.clone(),
                                        password: user_request.password.clone(),
                                        confirm_password: user_request.confirm_password.clone(),
                                        id: user.id.to_string(),
                                    }))
                                },
                                Err(e) => {
                                    eprintln!("Failed to create user: {}", e);
                                    HttpResponse::InternalServerError().body("Failed to create user")
                                },
                            }
                        } else {
                            HttpResponse::Forbidden().body("Insufficient permissions")
                        }
                    } else {
                        HttpResponse::Unauthorized().body("Invalid token structure")
                    }
                },
                Err(_) => HttpResponse::Unauthorized().body("Invalid JWT token"),
            }
        },
        Err(response) => response,
    }
}