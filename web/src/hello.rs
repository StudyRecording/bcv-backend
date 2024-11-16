use actix_web::{get, Responder};
use utils::res::ResultRes;

#[get("/hello")]
pub async fn hello_world() -> impl Responder {
    // HttpResponse::Ok().body("Hello World!")
    ResultRes::success("")
}