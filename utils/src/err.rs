use actix_web::{error, http::header::ContentType, HttpResponse};
use derive_more::derive::{Display, Error};

use crate::res::ResultRes;


#[derive(Debug, Display, Error)]
pub enum ResultErr {
    #[display("系统错误")]
    SysErr,

    #[display("业务错误: {msg}")]
    BizErr { msg: String },
}

impl ResultErr {

    /// 错误处理
    fn error_handler(&self) -> Result<String, serde_json::Error> {
        let body = match self {
            ResultErr::SysErr => serde_json::to_string(&ResultRes::sys_err("")),
            ResultErr::BizErr { msg } => serde_json::to_string(&ResultRes::biz_err("", msg.as_str())),
        };
        body
    }
}


impl error::ResponseError for ResultErr {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::OK
        
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        
        HttpResponse::build(self.status_code())
            .content_type(ContentType::json())
            .body(self.error_handler().unwrap())
    }
}




