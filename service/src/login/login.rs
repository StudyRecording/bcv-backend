use argon2::{Argon2, PasswordHash, PasswordVerifier};
use chrono::{Duration, Local};
use entity::{prelude::*, user};
use sea_orm::{entity::*, DatabaseConnection, EntityTrait, QueryFilter};
use tracing::error;
use utils::{err::ResultErr, token::get_token, user_info::UserInfo};

/// 登录业务
pub async fn login(
    account: String,
    password: String,
    db: &DatabaseConnection,
) -> Result<UserInfo, ResultErr> {
    // 查询数据
    let user = User::find()
        .filter(user::Column::Account.eq(account.as_str()))
        .one(db)
        .await;
    let user = match user {
        Ok(data) => data,
        Err(e) => {
            error!("登录获取用户信息失败, account:{}, e: {:?}", account, e);
            None
        }
    };

    if let Some(user) = user {
        let hash_password = user.password;
        // 验证
        let verify = Argon2::default().verify_password(
            password.as_bytes(),
            &PasswordHash::new(&hash_password).unwrap(),
        );

        if verify.is_ok() {
            // 生成token
            // todo!("生成secret");
            let access_exp = Local::now() + Duration::hours(2);
            let access_token = get_token(
                user.id,
                access_exp.timestamp(),
                account.clone(),
                "aaaa".into(),
            )
            .map_err(|_e| ResultErr::BizErr {
                msg: "生成access_token失败".into(),
            })?;
            let refresh_exp = Local::now() + Duration::days(7);
            let refresh_token = get_token(
                user.id,
                refresh_exp.timestamp(),
                account.clone(),
                "refresh_secret".into(),
            )
            .map_err(|_e| ResultErr::BizErr {
                msg: "生成refresh_token失败".into(),
            })?;
            let info = UserInfo {
                id: user.id,
                account,
                name: user.name,
                access_token,
                refresh_token,
            };
            return Ok(info);
        }
    }

    Err(ResultErr::BizErr {
        msg: "登录失败".into(),
    })
}

#[cfg(test)]
mod tests {

    use argon2::{
        password_hash::{PasswordHash, PasswordHasher, SaltString},
        Argon2, PasswordVerifier,
    };
    use base64::{prelude::BASE64_STANDARD, Engine};
    use crypto::digest::Digest;
    use rand::rngs::OsRng;
    use sha2::Sha256;

    #[test]
    fn test_token() {
        let salt = "Z123f";
        let mut digest = Sha256::new();
        digest.update(b"hu980512");
        digest.update(b"$");
        digest.update(salt.as_bytes());
        let hash = digest.finalize();
        println!("hash is {:x}, len is {}", hash, hash.len());
        println!("hash vec is {:?}", hash.to_vec());
        let len = "34f940737cc801e754ed32444b2d5fbca13d921a81c743e582d13ba333d46f2c".len();
        println!("actual len is {len}");

        let mut hash_str = String::new();
        BASE64_STANDARD.encode_string(hash, &mut hash_str);
        println!("hash_str is {}, len is {}", hash_str, hash_str.len());
        let decode = BASE64_STANDARD.decode(hash_str).unwrap();
        println!("decode is {:?}", decode);
    }

    #[test]
    fn test_argon2() {
        let password = b"admin";
        let hash_password = gen_hash_pwd(password);

        // 验证
        let verify = Argon2::default()
            .verify_password(password, &PasswordHash::new(&hash_password).unwrap());
        assert!(verify.is_ok());

        let n_p = b"hU980512";
        let h_e_pwd = gen_hash_pwd(n_p);
        let verify =
            Argon2::default().verify_password(password, &PasswordHash::new(&h_e_pwd).unwrap());
        assert!(verify.is_err());
    }

    /// 生成hash password
    fn gen_hash_pwd(password: &[u8]) -> String {
        let salt = SaltString::generate(&mut OsRng);
        println!("salt is {}, len is {}", salt, salt.len());
        let argon2 = Argon2::default();
        let hash_password = argon2.hash_password(password, &salt).unwrap().to_string();
        println!(
            "hash_password is {}, len is {}",
            hash_password,
            hash_password.len()
        );
        hash_password
    }
}

