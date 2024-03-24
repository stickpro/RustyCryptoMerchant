use sea_orm_migration::prelude::*;

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
                    .col(
                        ColumnDef::new(User::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::Email).string().not_null().unique_key())
                    .col(ColumnDef::new(User::Password).string().not_null())
                    .col(ColumnDef::new(User::ProcessingOwnerId).string().not_null())
                    .col(ColumnDef::new(User::IsActive).boolean().not_null())
                    .col(ColumnDef::new(User::Is2fa).boolean().not_null().default(false))
                    .col(ColumnDef::new(User::Secret2fa).string())
                    .col(ColumnDef::new(User::Phone).string().string_len(20))
                    .col(ColumnDef::new(User::Location).string().string_len(50))
                    .col(ColumnDef::new(User::Language).char().char_len(2))
                    .col(ColumnDef::new(User::CratedAt).timestamp())
                    .col(ColumnDef::new(User::UpdatedAt).timestamp())

                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx-user_email")
                    .table(User::Table)
                    .col(User::Email)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum User {
    Table,
    Id,
    Email,
    Password,
    ProcessingOwnerId,
    IsActive,
    Is2fa,
    Secret2fa,
    Phone,
    Location,
    Language,
    CratedAt,
    UpdatedAt
}
