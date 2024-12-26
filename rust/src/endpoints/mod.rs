pub mod user;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(user::login_endpoint);
    cfg.service(user::create_user_endpoint);
    cfg.service(user::update_user_endpoint);
    cfg.service(user::delete_user_endpoint);
    cfg.service(user::retrieve_all_users_endpoint);
    cfg.service(user::retrieve_user_endpoint);
}