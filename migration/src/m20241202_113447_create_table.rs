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
                    .table(User::Table)
                    .if_not_exists()
                    .col(pk_auto(User::Id))
                    .col(string_len_uniq(User::Account, 50))
                    .col(string_len(User::Name, 50))
                    .col(string_len(User::Password, 255))
                    .col(string_len(User::Salt, 50))
                    // .col(tiny_integer(User::Status))
                    .col(
                        ColumnDef::new(User::Status)
                            .tiny_integer()
                            .default(1)
                            .comment("状态：1正常，0删除")
                            .not_null()
                            .take()
                    )
                    .col(date_time(User::CreateTime))
                    .col(string(User::CreateUser))
                    .col(date_time(User::UpdateTime))
                    .col(string(User::UpdateUser))
                    .to_owned(),
            )
            .await

        
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Account,
    Name,
    Password,
    Salt,
    Status,
    CreateTime,
    CreateUser,
    UpdateTime,
    UpdateUser
}
