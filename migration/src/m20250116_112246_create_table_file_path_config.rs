use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(FilePathConfig::Table)
                    .if_not_exists()
                    .col(pk_auto(FilePathConfig::Id))
                    .col(
                        ColumnDef::new(FilePathConfig::Title)
                            .string()
                            .not_null()
                            .default("")
                            .comment("文件路径配置描述")
                            .take(),
                    )
                    .col(
                        ColumnDef::new(FilePathConfig::Type)
                            .integer()
                            .not_null()
                            .default(0)
                            .comment("配置类型：0书籍，1漫画，2视频")
                            .take()
                    )
                    .col(
                        ColumnDef::new(FilePathConfig::Path)
                            .string()
                            .not_null()
                            .default("")
                            .comment("配置目录")
                            .take()
                    )
                    .col(
                        ColumnDef::new(FilePathConfig::Status)
                            .tiny_integer()
                            .default(1)
                            .comment("状态：1正常，0删除")
                            .not_null()
                            .take(),
                    )
                    .col(date_time(FilePathConfig::CreateTime))
                    .col(string(FilePathConfig::CreateUser))
                    .col(date_time(FilePathConfig::UpdateTime))
                    .col(string(FilePathConfig::UpdateUser))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(FilePathConfig::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum FilePathConfig {
    Table,
    Id,
    Title,
    Type,
    Path,
    Status,
    CreateTime,
    CreateUser,
    UpdateTime,
    UpdateUser,
}
