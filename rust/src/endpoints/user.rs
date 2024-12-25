use actix_web::{web, Responder, HttpResponse, HttpRequest, post};
use crate::auth::*;
use rust::api::fetch_user;
use serde::{Deserialize, Serialize};
use rust::db::users::create_user;
use rust::middleware::check_sudo;



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
    match check_sudo(&req) {
        Ok(_) => {
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
                },
                Err(response) => response,
            }
        }