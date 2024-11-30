

use actix_web::{body::BoxBody, http::header::ContentType, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
pub struct ResultRes<T> {
    success: bool,
    data: Option<T>,
    code: u16,
    msg: Option<String>,
}

impl<T> ResultRes<T> {
    
    // 请求成功返回
    pub fn success(data: T) -> ResultRes<T> {
        Self {
            success: true,
            data: Some(data),
            code: 2000,
            msg: None,
        }
    }

    // 请求失败返回
    pub fn err(data: T, code: u16, msg: String) -> ResultRes<T> {
        Self { 
            success: false, 
            data: Some(data), 
            code, 
            msg: Some(msg)
        }
    }

    // 系统错误
    pub fn sys_err(data: T) -> ResultRes<T> {
        Self { success: false, data: Some(data), code: 500, msg: Some("系统错误".into()) }
    }

    // 业务错误
    pub fn biz_err(data: T, msg: String) -> ResultRes<T> {
        Self { success: false, data: Some(data), code: 2001, msg: Some(msg) }
    }

}

pub trait SuccessNoArg {
    fn success() -> Self;
}

impl<T> SuccessNoArg for ResultRes<T> {
    
    fn success() -> ResultRes<T> {
        Self {
            success: true,
            data: None,
            code: 2000,
            msg: None
        }
    }
}

impl<T: Serialize> Responder for ResultRes<T> {
    type Body = BoxBody;

    fn respond_to(self, req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        let _ = req;
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

