
use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    // 用户id
    sub: u16,
    // token过期时间
    exp: i64,
    // 签发时间
    iat: i64,
    // 用户姓名
    username: String
}

/// 获取token
pub fn get_token(sub: u16, exp: i64, username: String, secret: String) -> Result<String, jsonwebtoken::errors::Error> {

    let cliaims = Claims {sub, exp, iat: Utc::now().timestamp(), username};
    let header = Header {alg: Algorithm::HS512, ..Default::default()};
    let token = encode(&header, &cliaims, &EncodingKey::from_secret(secret.as_bytes()));
    token
}

/// 验证token
pub fn valid_token(token: String, secret: String) -> Result<jsonwebtoken::TokenData<Claims>, jsonwebtoken::errors::Error> {
    let data = decode::<Claims>(
        &token, 
        &DecodingKey::from_secret(secret.as_bytes()), 
        &Validation::new(Algorithm::HS512));
    data
}

#[cfg(test)]
mod tests {
    
    
    use chrono::{Days, Local};

    use super::*;

    #[test]
    fn test_token() {
        let iat = Local::now();
        let exp = iat.clone().checked_add_days(Days::new(1)).unwrap();
        
        let token = get_token(12233, exp.timestamp(), "hpc".into(), "qaz".into());
        let token = token.unwrap();
        println!("token is: {}", &token);

        let data = valid_token(token.clone(), "qaz".into());
        println!("data is: {:?}", data)
    }
}