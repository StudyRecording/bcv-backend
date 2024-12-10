
use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rand::Rng;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    // 用户id
    sub: i32,
    // token过期时间
    exp: i64,
    // 签发时间
    iat: i64,
    // 用户账户
    account: String
}

/// 获取token
pub fn get_token(sub: i32, exp: i64, account: String, secret: String) -> Result<String, jsonwebtoken::errors::Error> {

    let cliaims = Claims {sub, exp, iat: Utc::now().timestamp(), account};
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

/// 获取加密的盐
pub fn get_secret(num: i32) -> String {
    let mut rng = rand::thread_rng();
    let chars: Vec<char> = ('a'..='z')
        .chain('A'..='Z')
        .chain('0'..='9')
        .collect();
    (0..num).into_iter()
        .map(|_| chars.get(rng.gen_range(0..chars.len())).unwrap_or(&'0'))
        .collect::<String>()
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

    #[test]
    fn test_gen_salt() {
        let salt = get_secret(32);
        println!("salt is {salt}, len is {}", salt.len());
    }
}