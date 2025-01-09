use actix_web::web;
use super::login::login;


/// 登录、注册
pub fn login_config(cfg: &mut web::ServiceConfig) {
    cfg.service(login);
}