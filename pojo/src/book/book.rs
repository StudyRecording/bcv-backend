use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct BookInfoSaveParam {
    pub book_name: String,
    pub cover: String,
    pub path: String,
}
