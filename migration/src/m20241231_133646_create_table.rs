use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(BookInfo::Table)
                    .if_not_exists()
                    .col(pk_auto(BookInfo::Id))
                    .col(
                        ColumnDef::new(BookInfo::BookName)
                            .string_len(50)
                            .not_null()
                            .default("")
                            .comment("书名")
                            .take(),
                    )
                    .col(
                        ColumnDef::new(BookInfo::Cover)
                            .string_len(225)
                            .not_null()
                            .default("")
                            .comment("封面path")
                            .take(),
                    )
                    .col(
                        ColumnDef::new(BookInfo::Path)
                            .string_len(225)
                            .not_null()
                            .default("")
                            .comment("书籍路径")
                            .take(),
                    )
                    .col(
                        ColumnDef::new(BookInfo::Status)
                            .tiny_integer()
                            .default(1)
                            .comment("状态：1正常，0删除")
                            .not_null()
                            .take(),
                    )
                    .col(date_time(BookInfo::CreateTime))
                    .col(string(BookInfo::CreateUser))
                    .col(date_time(BookInfo::UpdateTime))
                    .col(string(BookInfo::UpdateUser))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(BookInfo::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum BookInfo {
    Table,
    Id,
    BookName,
    Cover,
    Path,
    Status,
    CreateTime,
    CreateUser,
    UpdateTime,
    UpdateUser,
}
