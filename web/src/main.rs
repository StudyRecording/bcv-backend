
use actix_web::{http::StatusCode, middleware::{from_fn, ErrorHandlers}, web, App, HttpServer};
use configure::config;
use log::log_middleware;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::util::SubscriberInitExt;
use utils::err::error_handler;

pub mod hello;
pub mod configure;
pub mod log;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // 日志
    let file_appender = RollingFileAppender::new(Rotation::DAILY, "web/file", "log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    log::log(non_blocking, "info".into()).init();

    // 启动应用
    HttpServer::new(|| {
        App::new()
            .wrap(
                ErrorHandlers::new()
                    .handler(StatusCode::INTERNAL_SERVER_ERROR, error_handler)
            )
            .wrap(from_fn(log_middleware))
            .service(web::scope("/api").configure(config))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await

}
