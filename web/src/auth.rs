use actix_web::{dev::ServiceRequest, error, Error};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use tracing::info;

/// 认证切面
pub async fn validator(req: ServiceRequest, credentials: Option<BearerAuth>) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    
    if req.path() == "/api/login" {
        return Ok(req);
    }
    
    info!("credentials is {credentials:?}");
    let Some(credentials) = credentials else {
        return Err((error::ErrorBadRequest("no bearer header"), req));
    };

    
    eprintln!("{credentials:?}");

    if credentials.token().contains('x') {
        return Err((error::ErrorBadRequest("token contains x"), req));
    }

    Ok(req)
}