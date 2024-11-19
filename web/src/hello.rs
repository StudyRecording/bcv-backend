use std::{fs::File, io::Read};

use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{error::UrlGenerationError, get, http::header::ContentDisposition, post, web::{Json, Path}, HttpResponse, Responder};
use serde_derive::{Deserialize, Serialize};
use utils::{err::ResultErr, res::ResultRes};

#[get("/hello")]
pub async fn hello_world() -> impl Responder {
    // HttpResponse::Ok().body("Hello World!")
    ResultRes::success("")
}

#[get("/path/{name}")]
pub async fn path(path: Path<String>) -> impl Responder {
    let name = path.into_inner();
    ResultRes::success(name)
}

#[derive(Serialize, Deserialize)]
struct Info {
    name: String,
    age: u8,
}

#[post("/post")]
pub async fn post(info: Json<Info>) -> impl Responder {
    let res_info = info.into_inner();
    ResultRes::success(res_info)
}

#[derive(Debug, MultipartForm)]
struct Uploadform{
    #[multipart(rename = "file")]
    file: TempFile
}

#[post("/save")]
pub async fn save_files(
    MultipartForm(form): MultipartForm<Uploadform>
) -> impl Responder {

    let mut buffer = Vec::new();

    let _ = form.file.file.into_file().read_to_end(&mut buffer);
    let content = String::from_utf8(buffer).expect("未获取文件内容");

    ResultRes::success(content)
}


#[get("/download")]
pub async fn download() -> impl Responder {
    let mut file = File::open("web/file/test.txt").expect("未找到下载文件");
    let mut buffer = Vec::new();
    let _ = file.read_to_end(&mut buffer).expect("读取文件失败");
    let content = String::from_utf8(buffer).expect("未获取文件内容");
    HttpResponse::Ok()
                .content_type("application/octet-stream")
                .append_header(ContentDisposition::attachment("测试.txt"))
                .body(content)
}


#[get("/exception/{pa}")]
pub async fn exception(pa: Path<String>) -> Result<impl Responder, ResultErr> {
    let path_param = pa.into_inner();
    if path_param == "sys_err" {
        return Err(ResultErr::SysErr);
    }
    if path_param == "biz_err" {
        return Err(ResultErr::BizErr { msg: "业务错误".to_string() })
    }
    Ok(ResultRes::success(""))
}

#[get("/er/{pa}")]
pub async fn er(pa: Path<String>) -> Result<impl Responder, UrlGenerationError> {
    if pa.into_inner() == "1" {
        return Err(UrlGenerationError::NotEnoughElements);
    }
    Ok(ResultRes::success(""))
}
