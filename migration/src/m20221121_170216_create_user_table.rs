use sea_orm_migration::{prelude::*, sea_query::extension::postgres::Type};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(UserProvider::Provider)
                    .values(vec![UserProvider::Google, UserProvider::Local])
                    .to_owned(),
            )
            .await?;
        let table = Table::create()
            .table(User::Table)
            .if_not_exists()
            .col(ColumnDef::new(User::Id).uuid().not_null().primary_key())
            .col(
                ColumnDef::new(User::Username)
                    .string()
                    .string_len(20)
                    .unique_key()
                    .not_null(),
            )
            .col(
                ColumnDef::new(User::Email)
                    .string()
                    .string_len(45)
                    .unique_key()
                    .not_null(),
            )
            .col(ColumnDef::new(User::PasswordHash).string())
            .col(
                ColumnDef::new(User::Provider)
                    .enumeration(
                        UserProvider::Provider,
                        vec![UserProvider::Google, UserProvider::Local],
                    )
                    .not_null(),
            )
            .col(
                ColumnDef::new(User::CreatedAt)
                    .timestamp()
                    .not_null()
                    .default("now()"),
            )
            .col(ColumnDef::new(User::UpdatedAt).timestamp().null())
            .col(ColumnDef::new(User::DeletedAt).timestamp().null())
            .index(
                Index::create()
                    .unique()
                    .name("idx-user-deleted-at")
                    .col(User::DeletedAt),
            )
            .to_owned();

        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().if_exists().table(User::Table).to_owned())
            .await?;
        manager
            .drop_type(
                Type::drop()
                    .if_exists()
                    .name(UserProvider::Provider)
                    .to_owned(),
            )
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum User {
    Table,
    Id,
    Username,
    Email,
    PasswordHash,
    Provider,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}

#[derive(Iden)]
enum UserProvider {
    Provider,
    Google,
    Local,
}
