use actix_web::web;

use super::user_login;

/// 书籍配置
pub fn login_config(cfg: &mut web::ServiceConfig) {
    cfg.service(user_login);
}