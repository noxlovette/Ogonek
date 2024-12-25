pub mod user;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(user::login_endpoint);
    cfg.service(user::create_user_endpoint);
}