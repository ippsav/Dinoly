use crate::m20221121_170216_create_user_table::User;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(Url::Table)
            .if_not_exists()
            .col(ColumnDef::new(Url::Id).uuid().not_null().primary_key())
            .col(ColumnDef::new(Url::Name).string().not_null().string_len(30))
            .col(ColumnDef::new(Url::Slug).string().not_null().unique_key())
            .col(ColumnDef::new(Url::RedirectTo).text().not_null())
            .col(ColumnDef::new(Url::OwnerId).uuid().not_null())
            .col(ColumnDef::new(Url::CreatedAt).timestamp().not_null())
            .col(ColumnDef::new(Url::UpdatedAt).timestamp().null())
            .col(ColumnDef::new(Url::DeletedAt).timestamp().null())
            .index(
                Index::create()
                    .unique()
                    .name("idx-url-deleted-at")
                    .col(Url::DeletedAt),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("FK_user_urls_key")
                    .from(Url::Table, Url::OwnerId)
                    .to(User::Table, User::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().if_exists().table(Url::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Url {
    Table,
    Id,
    Name,
    Slug,
    RedirectTo,
    OwnerId,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
