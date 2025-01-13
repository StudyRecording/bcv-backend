use actix_web::{body::MessageBody, dev::ServiceResponse, error, http::{header::{self, ContentType}, StatusCode}, middleware::ErrorHandlerResponse, HttpResponse};
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
        match self {
            ResultErr::SysErr => serde_json::to_string(&ResultRes::sys_err("")),
            ResultErr::BizErr { msg } => serde_json::to_string(&ResultRes::biz_err("", msg.into())),
        }
    }
}


impl error::ResponseError for ResultErr {
    fn status_code(&self) -> StatusCode {
        match self {
            ResultErr::SysErr => StatusCode::INTERNAL_SERVER_ERROR,
            ResultErr::BizErr { msg: _ } => StatusCode::OK,
        }
        
        
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        
        HttpResponse::build(self.status_code())
            .content_type(ContentType::json())
            .body(self.error_handler().unwrap())
    }
}


/// 错误全局处理程序
pub fn error_handler<B>(res: ServiceResponse<B>) -> error::Result<ErrorHandlerResponse<B>> {
    
    let (req, res) = res.into_parts();
    
    let res = ServiceResponse::new(req, res)
        .map_body(|a, _body| {
            a.headers_mut()
                .insert(header::CONTENT_TYPE, header::HeaderValue::from_static("application/json"));
            serde_json::to_string(&ResultRes::sys_err("")).unwrap().boxed()
        })
        .map_into_right_body();
    
    Ok(ErrorHandlerResponse::Response(res))
}




