use crate::m20241231_133646_create_table::BookInfo;
use sea_orm_migration::{prelude::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        let has_user_id = manager.has_column("book_info", "user_id").await?;

        if has_user_id {
            return Ok(());
        }

        // 不存在，则新增
        manager
            .alter_table(
                Table::alter()
                    .table(BookInfo::Table)
                    .add_column(
                        ColumnDef::new(Alias::new("user_id"))
                            .integer()
                            .not_null()
                            .comment("用户id")
                            .default(0),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();
        //
        // manager
        //     .drop_table(Table::drop().table(Post::Table).to_owned())
        //     .await
        Ok(())
    }
}
