use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        // 
        // // 判断表是否存在
        // let exist = manager.has_table("file_path_config").await?;
        // if !exist {
        //     return Ok(());
        // }
        // 
        // // 判断配置信息是否存在
        // let db = manager.get_connection();
        // let config_vec = FilePathConfig::find().all(db).await?;
        // 
        // if !config_vec.is_empty() { 
        //     return Ok(());
        // }
        // 
        // // 生成配置数据
        // let home_dir = dirs::home_dir().unwrap();
        // let root_dir = home_dir.join("BCV").to_str().unwrap().to_string();
        // 
        // // if !root_dir.exists() || !root_dir.is_dir() {
        // //     let _ = create_dir_all(root_dir);
        // // }
        // let book_dir = format!("{}/{}", root_dir.clone(), "books");
        // let comic_dir = format!("{}/{}", root_dir.clone(), "comic");
        // let video_dir = format!("{}/{}", root_dir.clone(), "video");
        // 
        // let book_model = file_path_config::ActiveModel { id: Set(1), title: Set("书籍目录".to_string()), r#type: Set(0), path: Set(book_dir.to_string()),
        //     status: Set(1),
        //     create_user: Set("system".into()),
        //     create_time: Set(Local::now().naive_local()),
        //     update_user: Set("system".into()),
        //     update_time: Set(Local::now().naive_local()) };
        // let comic_model = file_path_config::ActiveModel { id: Set(2), title: Set("漫画目录".to_string()), r#type: Set(1), path: Set(comic_dir.to_string()), status: Set(1),
        //     create_user: Set("system".into()),
        //     create_time: Set(Local::now().naive_local()),
        //     update_user: Set("system".into()),
        //     update_time: Set(Local::now().naive_local()) };
        // let video_model = file_path_config::ActiveModel { id: Set(3), title: Set("视频目录".to_string()), r#type: Set(2), path: Set(video_dir.to_string()), status: Set(1),
        //     create_user: Set("system".into()),
        //     create_time: Set(Local::now().naive_local()),
        //     update_user: Set("system".into()),
        //     update_time: Set(Local::now().naive_local()) };
        // 
        // let models = vec![book_model, comic_model, video_model];
        // 
        // FilePathConfig::insert_many(models)
        //     .exec(db)
        //     .await?;

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
