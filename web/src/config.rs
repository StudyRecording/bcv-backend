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
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub server: Server,
    pub db: DB,
    pub log: Log,
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
        Ok(config.unwrap())
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