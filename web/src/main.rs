use actix_web::{web, App, HttpServer};
use configure::config;

pub mod hello;
pub mod configure;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
            .service(web::scope("/api").configure(config))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await

}
