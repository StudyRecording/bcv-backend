use actix_web::{http::StatusCode, middleware::ErrorHandlers, web, App, HttpServer};
use configure::config;
use utils::err::error_handler;

pub mod hello;
pub mod configure;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

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
