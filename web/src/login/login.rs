use actix_web::{post, web, Responder};
use chrono::{Days, Local};
use serde::{Deserialize, Serialize};
use utils::{err::ResultErr, res::ResultRes, token::get_token};


#[derive(Debug, Serialize, Deserialize)]
struct UserInfo {
    username: String,
    password: String
}

#[post("/login")]
pub async fn login(info: web::Json<UserInfo>) -> Result<impl Responder, ResultErr> {

    let user_info = info.into_inner();
    // 1.根据username查出数据库中的加密后的password和盐salt，获取secret.
    let pwd = "aaaaaaaaa";
    let salt = "Z3slt";
    let secret = "AAbb123";
    // 2.比较密码
    // let password = encry(pwd, salt);
    let password = "aaaaaaaaa";
    if pwd == password {
        // 生成token,  过期时间exp, token有效期为1day
        let user_id = 1u16;
        match get_token(
            user_id, 
            Local::now().checked_add_days(Days::new(1)).unwrap().timestamp(), 
            user_info.username, 
            secret.into()
        ) {
            Ok(token) => Ok(ResultRes::success(token)),
            Err(_) => Err(ResultErr::BizErr { msg: "获取token事变".into() }),
        }
    } else {
        Err(ResultErr::BizErr { msg: "用户名或密码错误".into() })
    }

}