

use actix_web::{body::BoxBody, http::header::ContentType, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
pub struct ResultRes<'a, T: 'a> {
    success: bool,
    data: T,
    code: u16,
    msg: &'a str,
}

impl<'a, T> ResultRes<'a, T> {

    // 请求成功返回
    pub fn success(data: T) -> ResultRes<'a, T> {
        Self {
            success: true,
            data,
            code: 2000,
            msg: "",
        }
    }

    // 请求失败返回
    pub fn err(data: T, code: u16, msg: &'a str) -> ResultRes<'a, T> {
        Self { success: false, data, code, msg }
    }

    // 系统错误
    pub fn sys_err(data: T) -> ResultRes<'a, T> {
        Self { success: false, data, code: 500, msg: "系统错误" }
    }

    // 业务错误
    pub fn biz_err(data: T, msg: &'a str) -> ResultRes<'a, T> {
        Self { success: false, data, code: 2001, msg }
    }

}

impl<'a, T: Serialize> Responder for ResultRes<'a, T> {
    type Body = BoxBody;

    fn respond_to(self, req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        let _ = req;
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

