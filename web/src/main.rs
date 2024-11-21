use std::io;

use actix_web::{http::StatusCode, middleware::ErrorHandlers, web, App, HttpServer};
use configure::config;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use utils::err::error_handler;

pub mod hello;
pub mod configure;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // 日志
    let stdout_layer = tracing_subscriber::fmt::layer()
        .pretty()
        .with_writer(io::stdout);

    // let file_appender = tracing_appender::rolling::daily("web/file/", "log");
    let file_appender = RollingFileAppender::new(Rotation::DAILY, "web/file", "log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let file_layer = tracing_subscriber::fmt::layer()
        .with_ansi(false)
        .with_writer(non_blocking);

    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(env_filter)
        .with(stdout_layer)
        .with(file_layer)
        .init();

    // 启动应用
    HttpServer::new(|| {
        App::new()
            .wrap(
                ErrorHandlers::new()
                    .handler(StatusCode::INTERNAL_SERVER_ERROR, error_handler)
            )
            .service(web::scope("/api").configure(config))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await

}
