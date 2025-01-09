use entity::{path_info, prelude::PathInfo};
use sea_orm::{entity::*, DatabaseConnection, EntityTrait, QueryFilter};
use utils::err::ResultErr;


/// 获取path认证类型
pub async fn get_path_type(db: &DatabaseConnection, path: &str) -> Result<i8, ResultErr> {
    
    let path_info = PathInfo::find()
        .filter(path_info::Column::Path.eq(path))
        .filter(path_info::Column::Status.eq(1))
        .one(db)
        .await;

    match path_info {
        Ok(info) => {
            if info.is_none() {
                return Ok(0);
            }
            return Ok(info.unwrap().r#type)
        },
        Err(e) => Err(ResultErr::BizErr { msg: e.to_string() }),
    }
}