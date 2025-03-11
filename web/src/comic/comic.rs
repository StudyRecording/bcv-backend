use actix_web::{get, HttpResponse, Responder};
use actix_web::web::Data;
use service::AppState;
use utils::err::ResultErr;

#[get("/get")]
pub async fn get_comic_picture(_data: Data<AppState>) -> Result<impl Responder, ResultErr> {
    let result = service::comic::get_zip_picture(1).await?;
    Ok(HttpResponse::Ok().content_type("image/jpeg").body(result))
}