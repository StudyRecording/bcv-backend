
use actix_web::{dev::ServiceRequest, error, Error, HttpMessage};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use service::AppState;
use tracing::info;
use utils::token::valid_token;

/// 认证切面
pub async fn validator(req: ServiceRequest, credentials: Option<BearerAuth>) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    

    let path = req.path();
    let conn = &req.app_data::<AppState>().unwrap().conn;
    // 根据path查询url，获取是否需要认证, 0:不需要，1需要，2皆可
    let need_auth = service::base::get_path_type(conn, path).await.unwrap_or(0);
    
    // 不需要认证
    if need_auth == 0 || (need_auth == 2 && credentials.is_none()) {
        return Ok(req);
    }

    if need_auth == 1 && credentials.is_none() {  
        eprintln!("{credentials:?}");
        return Err((error::ErrorBadRequest("no bearer header"), req));
    }
    
    info!("credentials is {credentials:?}");
    let data = valid_token(credentials.unwrap().token().into(), "aaaa".into());
    if data.is_ok() {
        info!("login auth is: {:?}", data);
        let user_id = data.unwrap().claims.sub;
        req.extensions_mut().insert(user_id);
        return Ok(req);
    }

    // if credentials.token().contains('x') {
    //     return Err((error::ErrorBadRequest("token contains x"), req));
    // }

    Ok(req)
}