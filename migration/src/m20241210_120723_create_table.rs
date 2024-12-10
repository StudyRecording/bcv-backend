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
                    .table(SecretInfo::Table)
                    .if_not_exists()
                    .col(pk_auto(SecretInfo::Id))
                    .col(
                        ColumnDef::new(SecretInfo::AccessSecret)
                            .string_len(32)
                            .not_null()
                            .default("")
                            .comment("access_token的secret")
                            .take()
                    )
                    .col(
                        ColumnDef::new(SecretInfo::RefreshSecret)
                            .string_len(32)
                            .not_null()
                            .default("")
                            .comment("refresh_token的secret")
                            .take()
                    )
                    .col(
                        ColumnDef::new(SecretInfo::AccessEndTime)
                            .date_time()
                            .comment("access token的secret的有效截至时间")
                            .take()
                    )
                    .col(
                        ColumnDef::new(SecretInfo::RefreshEndTime)
                            .date_time()
                            .comment("refresh token的secret的有效截至时间")
                            .take()
                    )
                    .col(
                        ColumnDef::new(SecretInfo::Status)
                            .tiny_integer()
                            .default(1)
                            .comment("状态：1正常，0删除")
                            .not_null()
                            .take()
                    )
                    .col(date_time(SecretInfo::CreateTime))
                    .col(string(SecretInfo::CreateUser))
                    .col(date_time(SecretInfo::UpdateTime))
                    .col(string(SecretInfo::UpdateUser))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(SecretInfo::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum SecretInfo {
    Table,
    Id,
    AccessSecret,
    RefreshSecret,
    AccessEndTime,
    RefreshEndTime,
    Status,
    CreateTime,
    CreateUser,
    UpdateTime,
    UpdateUser
}
