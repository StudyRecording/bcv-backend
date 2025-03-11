use crate::local_file_storage::file_info::FileInfo;
use actix_multipart::form::tempfile::TempFile;
use chrono::Local;
use entity::file_path_config;
use entity::file_path_config::Column;
use entity::prelude::FilePathConfig;
use sea_orm::ActiveValue::Set;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use std::fs::{create_dir_all, read_dir, File};
use std::io::{Read, Write};
use tracing::info;
use utils::err::ResultErr;

/// 保存文件
pub async fn save_file(
    file: TempFile,
    category: i32,
    db: &DatabaseConnection,
) -> Result<FileInfo, ResultErr> {
    let dir = get_file_dir(category, db).await?;
    let file_name = file.file_name.unwrap();
    let mut file = file.file.into_file();

    // 验证文件名是否相同
    if let Ok(entities) = read_dir(dir.as_str()) {
        for dir_entry in entities.flatten() {
            let entry_name = dir_entry.file_name().to_str().unwrap_or("").to_string();
            info!("entry name: {}", entry_name);
            if entry_name == file_name {
                return Err(ResultErr::BizErr {
                    msg: "存在相同文件名".into(),
                });
            }
        }
    }

    let file_path = dir + "/" + file_name.as_str();
    info!("new file path: {}", file_path);

    let new_file = File::create(file_path.as_str());
    if let Ok(mut new_file) = new_file {
        let mut buf = Vec::new();
        let _ = file.read_to_end(&mut buf);
        let _ = new_file.write_all(&buf);
        return Ok(FileInfo {
            path: file_path,
            file_name,
            category,
        });
    }

    Err(ResultErr::BizErr {
        msg: "文件上传失败".into(),
    })
}

/// 获取文件目录
pub async fn get_file_dir(category: i32, db: &DatabaseConnection) -> Result<String, ResultErr> {
    let file_path_config = FilePathConfig::find()
        .filter(Column::Type.eq(category))
        .filter(Column::Status.eq(1))
        .one(db)
        .await;
    match file_path_config {
        Ok(config) => Ok(config.unwrap().path),
        Err(e) => Err(ResultErr::BizErr { msg: e.to_string() }),
    }
}

/// 初始化文件内容存储
pub async fn init_file_config(
    db: &DatabaseConnection,
    book_dir: String,
    comic_dir: String,
    video_dir: String,
) -> Result<(), ResultErr> {
    
    // 初始化数据库
    init_file_config_db(db, book_dir.clone(), comic_dir.clone(), video_dir.clone()).await?;
    
    // 初始化文件
    let book = create_dir_all(book_dir);
    if book.is_err() { 
        return Err(ResultErr::BizErr {msg: "初始化书籍目录失败".into()});
    }
    
    let comic = create_dir_all(comic_dir);
    if comic.is_err() {
        return Err(ResultErr::BizErr {msg: "初始化漫画目录失败".into()});
    }
    
    let video = create_dir_all(video_dir);
    if video.is_err() {
        return Err(ResultErr::BizErr {msg: "初始化视频目录失败".into()});
    }
    
    Ok(())
}

/// 初始化文件内容存储目录数据
async fn init_file_config_db(
    db: &DatabaseConnection,
    book_dir: String,
    comic_dir: String,
    video_dir: String,
) -> Result<(), ResultErr> {
    let config_vec = FilePathConfig::find().all(db).await;
    if config_vec.is_err() {
        return Err(ResultErr::BizErr {
            msg: "文件内容存储配置查询失败".into(),
        });
    }

    if !config_vec.unwrap().is_empty() {
        return Ok(());
    }

    let book_model = file_path_config::ActiveModel {
        id: Set(1),
        title: Set("书籍目录".to_string()),
        r#type: Set(0),
        path: Set(book_dir),
        status: Set(1),
        create_user: Set("system".into()),
        create_time: Set(Local::now().naive_local()),
        update_user: Set("system".into()),
        update_time: Set(Local::now().naive_local()),
    };
    let comic_model = file_path_config::ActiveModel {
        id: Set(2),
        title: Set("漫画目录".to_string()),
        r#type: Set(1),
        path: Set(comic_dir),
        status: Set(1),
        create_user: Set("system".into()),
        create_time: Set(Local::now().naive_local()),
        update_user: Set("system".into()),
        update_time: Set(Local::now().naive_local()),
    };
    let video_model = file_path_config::ActiveModel {
        id: Set(3),
        title: Set("视频目录".to_string()),
        r#type: Set(2),
        path: Set(video_dir),
        status: Set(1),
        create_user: Set("system".into()),
        create_time: Set(Local::now().naive_local()),
        update_user: Set("system".into()),
        update_time: Set(Local::now().naive_local()),
    };
    let models = vec![book_model, comic_model, video_model];
    match FilePathConfig::insert_many(models).exec(db).await {
        Ok(_) => Ok(()),
        Err(e) => Err(ResultErr::BizErr { msg: e.to_string() }),
    }
}
