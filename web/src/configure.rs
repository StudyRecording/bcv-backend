use actix_web::web;

use crate::{hello::{create, download, er, exception, hello_world, path, post, query_info, save_files}, login::login::login};


/**
 * 配置路由
 */
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(hello_world)
        .service(path)
        .service(post)
        .service(save_files)
        .service(download)
        .service(exception)
        .service(er)
        .service(query_info)
        .service(create)
        .service(login);
}


