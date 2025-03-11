use actix::Addr;
use sea_orm::DatabaseConnection;
use crate::GlobalData;

#[derive(Debug, Clone)]
pub struct AppState {
    // 数据库连接
    pub conn: DatabaseConnection,
    
    // 全局数据
    pub addr: Addr<GlobalData>,
}

