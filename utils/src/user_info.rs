use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct UserInfo {
    pub id: i32,
    pub account: String,
    pub name: String,
    pub access_token: String,
    pub refresh_token: String,
}
