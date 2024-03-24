use sea_orm_migration::prelude::*;
use crate::migration::{m20240322_122454_create_user_table::User};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(Store::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Store::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Store::UserId).uuid().not_null())
                    .foreign_key(ForeignKey::create()
                        .name("fk-user-id")
                        .from(Store::Table, Store::UserId)
                        .to(User::Table, User::Id))
                    .col(ColumnDef::new(Store::Name).string().not_null())
                    .col(ColumnDef::new(Store::Site).string().not_null())
                    .col(ColumnDef::new(Store::CurrencyId).string().not_null())
                    .col(ColumnDef::new(Store::RateSource).string().not_null())
                    .col(ColumnDef::new(Store::ReturnUrl).string())
                    .col(ColumnDef::new(Store::SuccessUrl).string())
                    .col(ColumnDef::new(Store::RateScale).string())
                    .col(ColumnDef::new(Store::Status).boolean().default(true))
                    .col(ColumnDef::new(Store::CratedAt).timestamp())
                    .col(ColumnDef::new(Store::UpdatedAt).timestamp())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Store::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Store {
    Table,
    Id,
    UserId,
    Name,
    Site,
    CurrencyId,
    RateSource,
    ReturnUrl,
    SuccessUrl,
    RateScale,
    Status,
    CratedAt,
    UpdatedAt,
}
