use chrono::{Days, Local};
use entity::{prelude::*, secret_info};
use sea_orm::{PaginatorTrait, Set};
use sea_orm_migration::{prelude::*, sea_orm::entity::*};
use utils::token::get_secret;
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let db = manager.get_connection();
        let count = SecretInfo::find().count(db).await?;
        if count > 0 {
            return Ok(());
        }

        secret_info::ActiveModel {
            id: Set(1),
            access_secret: Set(get_secret(32)),
            refresh_secret: Set(get_secret(32)),
            access_end_time: Set(Local::now().naive_local().checked_add_days(Days::new(1))),
            refresh_end_time: Set(Local::now().naive_local().checked_add_days(Days::new(30))),
            status: Set(1),
            create_user: Set("System".into()),
            create_time: Set(Local::now().naive_local()),
            update_user: Set("System".into()),
            update_time: Set(Local::now().naive_local()),
        }
        .insert(db)
        .await?;

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        Ok(())
    }
}


