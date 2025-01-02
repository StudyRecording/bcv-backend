use actix_web::{post, web, Responder};
use serde::{Deserialize, Serialize};
use service::AppState;
use utils::{err::ResultErr, res::ResultRes};

#[derive(Debug, Serialize, Deserialize)]
struct UserInfo {
    account: String,
    password: String,
}

#[post("/login")]
pub async fn login(
    info: web::Json<UserInfo>,
    data: web::Data<AppState>,
) -> Result<impl Responder, ResultErr> {
    let user_info = info.into_inner();
    let info = service::login(user_info.account, user_info.password, &data.conn).await?;

    Ok(ResultRes::success(info))
}

