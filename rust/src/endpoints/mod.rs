use actix_web::{web, Responder, HttpResponse, post};
use crate::auth::generate_jwt;
use uuid::Uuid;
use rust::api::fetch_user;

#[derive(serde::Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[post("/login")]
async fn login_endpoint(login_request: web::Json<LoginRequest>) -> impl Responder {
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

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(login_endpoint);
}