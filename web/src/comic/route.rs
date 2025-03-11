use actix_web::web;
use crate::comic::get_comic_picture;

pub fn comic_config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_comic_picture);
}