use sea_orm::DatabaseConnection;

#[derive(Debug, Clone)]
pub struct AppState {
    // 数据库连接
    pub conn: DatabaseConnection,
}

