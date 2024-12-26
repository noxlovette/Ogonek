use actix_web::{web, Responder, HttpResponse, HttpRequest, post};
use rust::schema::users::password;
use crate::auth::*;
use rust::db::users::{retrieve_user, update_user};
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

#[derive(Deserialize)]
pub struct UserUpdateRequest {
    user_id: String,
    username: Option<String>,
    password: Option<String>,
    confirm_password: Option<String>,
    is_superuser: Option<bool>,
    first_name: Option<String>,
    last_name: Option<String>,
    email: Option<String>,

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
    match retrieve_user(&login_request.username, &login_request.password) {
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

use rust::models::users::UserUpdate;

#[post("/update_user")]
pub async fn update_user_endpoint(req: HttpRequest, user_request: web::Json<UserUpdateRequest>) -> impl Responder {
    if user_request.password.is_some() {
        if user_request.confirm_password.is_none() {
            return HttpResponse::BadRequest().body("Confirm password must be provided when changing password");
        }
        if let (Some(passwordstring), Some(confirm_password)) = (&user_request.password, &user_request.confirm_password) {
            if passwordstring != confirm_password {
                return HttpResponse::BadRequest().body("Passwords do not match");
            }
        }
    }

    // Check for sudo privileges
    if let Err(response) = check_sudo(&req) {
        return response;
    }

    // Prepare update data, only including fields that are actually updated
    let update_data = UserUpdate {
        username: user_request.username.clone(), 
        password: user_request.password.clone(),
        is_superuser: user_request.is_superuser.clone(),
        first_name: user_request.first_name.clone(),
        last_name: user_request.last_name.clone(),
        email: user_request.email.clone(),
        last_login: None,
        is_staff: None,
        is_active: None,
    };

    // Update user
    match update_user(user_request.user_id.clone(), update_data) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => {
            log::error!("Failed to update user: {}", e);
            HttpResponse::InternalServerError().body(format!("Failed to update user: {}", e))
        }
    }
}