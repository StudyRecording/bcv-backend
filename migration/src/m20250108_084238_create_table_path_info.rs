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
                    .table(PathInfo::Table)
                    .if_not_exists()
                    .col(pk_auto(PathInfo::Id))
                    .col(
                        ColumnDef::new(PathInfo::Name)
                            .string_len(50)
                            .not_null()
                            .default("")
                            .comment("请求名")
                            .take(),
                    )
                    .col(
                        ColumnDef::new(PathInfo::Type)
                            .tiny_integer()
                            .not_null()
                            .default(0)
                            .comment("path类型，0-不需认证，1-需要认证，2-认证和不认证场景都可")
                            .take(),
                    )
                    .col(
                        ColumnDef::new(PathInfo::Path)
                            .string_len(225)
                            .not_null()
                            .default("")
                            .comment("path路径")
                            .take(),
                    )
                    .col(
                        ColumnDef::new(PathInfo::Status)
                            .tiny_integer()
                            .default(1)
                            .comment("状态：1正常，0删除")
                            .not_null()
                            .take(),
                    )
                    .col(date_time(PathInfo::CreateTime))
                    .col(string(PathInfo::CreateUser))
                    .col(date_time(PathInfo::UpdateTime))
                    .col(string(PathInfo::UpdateUser))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(PathInfo::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum PathInfo {
    Table,
    Id,
    Name,
    Type,
    Path,
    Status,
    CreateTime,
    CreateUser,
    UpdateTime,
    UpdateUser,
}
