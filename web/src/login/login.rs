use actix_web::{post, web, Responder};
use serde::{Deserialize, Serialize};
use service::AppState;
use tracing::info;
use utils::{err::ResultErr, res::ResultRes};

#[derive(Debug, Serialize, Deserialize)]
struct UserInfo {
    account: String,
    password: String,
}

/// 登录接口
#[post("/login")]
pub async fn user_login(
    info: web::Json<UserInfo>,
    data: web::Data<AppState>,
) -> Result<impl Responder, ResultErr> {
    info!("登录接口....");
    let user_info = info.into_inner();
    let info = service::login::login(user_info.account, user_info.password, &data.conn).await?;

    Ok(ResultRes::success(info))
}

