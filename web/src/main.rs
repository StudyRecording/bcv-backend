use std::time::Duration;

use actix_web::{
    http::StatusCode,
    middleware::{from_fn, ErrorHandlers},
    web, App, HttpServer,
};
use actix_web_httpauth::middleware::HttpAuthentication;
use auth::validator;
use configure::hello_config;
use log::log_middleware;
use login::login;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database};
use service::AppState;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::util::SubscriberInitExt;
use utils::err::error_handler;

pub mod auth;
pub mod book;
pub mod configure;
pub mod hello;
pub mod log;
pub mod login;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 日志
    let file_appender = RollingFileAppender::new(Rotation::DAILY, "web/file", "log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    log::log(non_blocking, "info".into()).init();

    // 连接数据库
    // let db: DatabaseConnection = Database::connect("sqlite://db/bcv.db?mode=rwc").await?;

    let mut opt = ConnectOptions::new("sqlite://db/bcv.db?mode=rwc");
    opt.max_connections(7)
        .min_connections(3)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(false)
        // .sqlx_logging_level(log::LevelFilter::Info)
        .set_schema_search_path("bcv"); // Setting default PostgreSQL schema

    let conn = match Database::connect(opt).await {
        Ok(db) => db,
        Err(e) => panic!("获取数据库连接失败, 原因: {}", e.to_string()),
    };

    // 如果表不存在则创建
    Migrator::up(&conn, None).await.unwrap();

    let app_data = AppState { conn };

    // 启动应用
    HttpServer::new(move || {
        App::new()
            .wrap(ErrorHandlers::new().handler(StatusCode::INTERNAL_SERVER_ERROR, error_handler))
            .app_data(web::Data::new(app_data.clone()))
            .wrap(from_fn(log_middleware))
            .service(
                web::scope("/api")
                    .service(login)
                    .service(web::scope("/book").configure(book::route::book_config))
                    .service(
                        web::scope("/test")
                            .wrap(HttpAuthentication::with_fn(validator))
                            .configure(hello_config),
                    ),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
