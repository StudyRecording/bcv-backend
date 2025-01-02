use chrono::Local;
use entity::{prelude::*, user};
use sea_orm::{QueryFilter, Set};
use sea_orm_migration::{prelude::*, sea_orm::entity::*};


#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let user_vec = User::find()
            .filter(user::Column::Account.contains("admin"))
            .all(db)
            .await?;
        
        if user_vec.len() > 0 {
            return Ok(());
        }

        user::ActiveModel {
            id: Set(1),
            account: Set("admin".to_owned()),
            name: Set("Admin".to_owned()),
            password: Set("$argon2id$v=19$m=19456,t=2,p=1$yYEngmErBM3yOUdJiUxVgg$cpMABCeywcfs2H14FyG3z7oPDOiRD2pn3fMKOJQ3QC8".to_owned()),
            salt: Set("yYEngmErBM3yOUdJiUxVgg".to_owned()),
            status: Set(1),
            create_time: Set(Local::now().naive_local()),
            create_user: Set("System".to_owned()),
            update_time: Set(Local::now().naive_local()),
            update_user: Set("System".to_owned()),
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

