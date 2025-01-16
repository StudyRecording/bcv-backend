use std::time::Duration;

use crate::config::{Config, Storage};
use actix_web::{
    http::StatusCode,
    middleware::{from_fn, ErrorHandlers},
    web, App, HttpServer,
};
use actix_web_httpauth::middleware::HttpAuthentication;
use auth::validator;
use log::log_middleware;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database};
use service::AppState;
use test_route::hello_config;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::util::SubscriberInitExt;
use service::local_file_storage::init_file_config;
use utils::err::error_handler;

pub mod auth;
pub mod book;
pub mod hello;
pub mod log;
pub mod login;
pub mod test_route;
mod config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // 获取配置
    let config = match Config::new("web/config.toml") {
        Ok(config) => config,
        Err(e) => panic!("读取配置文件失败, 原因: {}", e)
    };

    // 日志
    let log_config = config.log;
    let file_appender = RollingFileAppender::new(Rotation::DAILY, log_config.dir, log_config.log_file_prefix);
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    log::log(non_blocking, log_config.level).init();

    // 数据库连接
    let db_config = config.db;
    let mut opt = ConnectOptions::new(db_config.url);
    opt.max_connections(db_config.max_connections)
        .min_connections(db_config.min_connections)
        .connect_timeout(Duration::from_secs(db_config.connect_timeout))
        .acquire_timeout(Duration::from_secs(db_config.acquire_timeout))
        .idle_timeout(Duration::from_secs(db_config.idle_timeout))
        .max_lifetime(Duration::from_secs(db_config.max_lifetime))
        .sqlx_logging(db_config.sqlx_logging)
        // .sqlx_logging_level(log::LevelFilter::Info)
        .set_schema_search_path(db_config.schema_search_path);

    let conn = match Database::connect(opt).await {
        Ok(db) => db,
        Err(e) => panic!("获取数据库连接失败, 原因: {}", e),
    };

    if db_config.migrator_up {
        // 如果表不存在则创建
        Migrator::up(&conn, None).await.unwrap();
    }
    
    // 初始化目录
    let Storage { root_dir: _, book_dir, comic_dir, video_dir } = config.storage;
    let init_storage = init_file_config(&conn, book_dir.unwrap(), comic_dir.unwrap(), video_dir.unwrap()).await;
    if init_storage.is_err() { 
        panic!("初始化文件存储目录失败");
    }
    
    let app_data = AppState { conn };

    // 启动应用
    let server_config = config.server;
    HttpServer::new(move || {
        App::new()
            .wrap(ErrorHandlers::new().handler(StatusCode::INTERNAL_SERVER_ERROR, error_handler))
            .app_data(web::Data::new(app_data.clone()))
            .wrap(HttpAuthentication::with_fn(validator))
            .wrap(from_fn(log_middleware))
            .service(
                web::scope("/api")
                    .service(web::scope("/user").configure(login::route::login_config))
                    .service(web::scope("/book").configure(book::route::book_config))
                    .service(web::scope("/test").configure(hello_config)),
            )
    })
    .workers(server_config.thread_num)
    .shutdown_timeout(server_config.shutdown_timeout)
    .bind((server_config.host, server_config.port))?
    .run()
    .await
}
