use actix_web::web;

use crate::hello::{download, er, exception, hello_world, path, post, query_info, save_files};

/**
 * 测试路由
 */
pub fn hello_config(cfg: &mut web::ServiceConfig) {
    cfg.service(hello_world)
        .service(path)
        .service(post)
        .service(save_files)
        .service(download)
        .service(exception)
        .service(er)
        .service(query_info);
}



