use serde::{Deserialize, Serialize};

/// 文件信息
#[derive(Serialize, Deserialize, Debug)]
pub struct FileInfo {
    // 路径
    pub path: String,
    // 文件名称
    pub file_name: String,
    // 文件类型：0书籍，1漫画，2视频
    pub category: i32
}

impl FileInfo {

    pub fn new(path: String, file_name: String, category: i32) -> FileInfo {
        FileInfo {path, file_name, category}
    }
}