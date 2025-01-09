use actix_web::web;

use super::book::{get_by_id, page_list, save};

/// 书籍配置
pub fn book_config(cfg: &mut web::ServiceConfig) {
    cfg.service(save)
        .service(get_by_id)
        .service(page_list);
}