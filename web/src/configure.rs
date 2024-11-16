use actix_web::web;

use crate::hello::hello_world;


/**
 * 配置路由
 */
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(hello_world);
}