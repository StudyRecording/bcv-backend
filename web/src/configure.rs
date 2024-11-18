use actix_web::web;

use crate::hello::{download, exception, hello_world, path, post, save_files};


/**
 * 配置路由
 */
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(hello_world)
        .service(path)
        .service(post)
        .service(save_files)
        .service(download)
        .service(exception);
}


