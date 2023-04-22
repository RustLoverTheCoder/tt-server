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
                    .table(Chat::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Chat::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Chat::FolderId).integer())
                    .col(ColumnDef::new(Chat::ChatType).string().not_null())
                    .col(ColumnDef::new(Chat::Title).string().not_null())
                    .col(ColumnDef::new(Chat::IsVerified).boolean().not_null())
                    .col(ColumnDef::new(Chat::MembersCount).integer().not_null())
                    .col(ColumnDef::new(Chat::IsSupport).boolean().not_null())
                    .col(ColumnDef::new(Chat::IsProtected).boolean().not_null())
                    .col(ColumnDef::new(Chat::RestrictionReason).string())
                    .col(
                        ColumnDef::new(Chat::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Chat::UpdatedAt)
                            .timestamp_with_time_zone(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Chat::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Chat {
    Table,
    Id,
    FolderId,
    ChatType,
    Title,
    IsVerified,
    MembersCount,
    IsSupport,
    IsProtected,
    RestrictionReason,
    CreatedAt,
    UpdatedAt,
}
