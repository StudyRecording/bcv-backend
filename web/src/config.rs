use std::fs::File;
use std::io::Read;
use serde_derive::{Deserialize, Serialize};
use tracing::error;
use utils::err::ResultErr;

/// 服务配置
#[derive(Serialize, Deserialize, Debug)]
pub struct Server {
    pub host: String,
    pub port: u16,
    pub thread_num: usize,
    pub shutdown_timeout: u64
}

/// 数据库配置
#[derive(Serialize, Deserialize, Debug)]
pub struct DB {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout: u64,
    pub acquire_timeout: u64,
    pub idle_timeout: u64,
    pub max_lifetime: u64,
    pub sqlx_logging: bool,
    pub schema_search_path: String,
    pub migrator_up: bool
}

/// 日志配置
#[derive(Serialize, Deserialize, Debug)]
pub struct Log {
    pub level: String,
    pub dir: String,
    pub log_file_prefix: String
}

/// 内容存储
#[derive(Serialize, Deserialize, Debug)]
pub struct Storage {
    pub root_dir: Option<String>,
    pub book_dir: Option<String>,
    pub comic_dir: Option<String>,
    pub video_dir: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub server: Server,
    pub db: DB,
    pub log: Log,
    pub storage: Storage,
}

impl Config {
    
    /// 获取配置信息
    pub fn new(path: &str) -> Result<Config, ResultErr> {
        let config_file = File::open(path);
        if config_file.is_err() { 
            return Err(ResultErr::BizErr {msg: "未找到配置文件".into()});
        }
        let mut config_file = config_file.unwrap();
        let mut config_content = String::new();
        match config_file.read_to_string(&mut config_content) {
            Ok(num) => num,
            Err(e) => {
                error!("读取配置文件失败, 失败原因: {}", e);
                return Err(ResultErr::BizErr {msg: "读取配置文件失败".into()});
            }
        };
        
        let config = toml::from_str::<Config>(&config_content);
        if config.is_err() {
            return Err(ResultErr::BizErr {msg: "配置文件解析失败".into()})
        }
        
        // 配置文件默认值设置
        let config = config.unwrap();
        let Config {server, db, log, storage} = config;
        let storage = default_storage_config(storage);
        
        Ok(Config {server, db, log, storage})
    }
}

/// 处理文件内容存储默认值
fn default_storage_config(storage: Storage) -> Storage {
    let Storage {root_dir, book_dir, comic_dir, video_dir} = storage;
    
    let home_dir = dirs::home_dir().unwrap().to_str().unwrap().to_string();
    let home_dir = home_dir.replace("\\", "/");
    let root_dir = root_dir.unwrap_or(home_dir + "/BCV");
    
    Storage {
        root_dir: Some(root_dir.clone()), 
        book_dir: Some(book_dir.unwrap_or(root_dir.clone() + "/books")), 
        comic_dir: Some(comic_dir.unwrap_or(root_dir.clone() + "/comic")), 
        video_dir: Some(video_dir.unwrap_or(root_dir.clone() + "/video")),
    }
}

#[cfg(test)]
mod tests {
    use crate::config::Config;

    #[test]
    fn test_config() {
        let config = Config::new("../web/config.toml");
        println!("{:?}", config);
    }
}