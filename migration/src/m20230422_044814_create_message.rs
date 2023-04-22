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
                    .table(Message::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Message::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Message::ChatId).uuid().not_null())
                    .col(ColumnDef::new(Message::ContentId).uuid().not_null())
                    .col(
                        ColumnDef::new(Message::Date)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Message::SenderId).uuid().not_null())
                    .col(ColumnDef::new(Message::ReplyToChatId).uuid())
                    .col(ColumnDef::new(Message::ReplyToMessageId).uuid())
                    .col(ColumnDef::new(Message::ReplyToTopMessageId).uuid())
                    .col(
                        ColumnDef::new(Message::SendingState)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Message::IsDeleting).boolean().not_null())
                    .col(ColumnDef::new(Message::PreviousLocalId).integer())
                    .col(ColumnDef::new(Message::Views).integer().not_null())
                    .col(ColumnDef::new(Message::Forwards).integer().not_null())
                    .col(ColumnDef::new(Message::IsEdited).boolean().not_null())
                    .col(
                        ColumnDef::new(Message::EditDate)
                            .timestamp_with_time_zone(),
                    )
                    .col(ColumnDef::new(Message::IsMentioned).boolean().not_null())
                    .col(ColumnDef::new(Message::GroupedId).uuid())
                    .col(ColumnDef::new(Message::IsScheduled).boolean().not_null())
                    .col(
                        ColumnDef::new(Message::IsFromScheduled)
                            .boolean()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Message::IsSilent).boolean().not_null())
                    .col(ColumnDef::new(Message::IsPinned).boolean().not_null())
                    .col(ColumnDef::new(Message::IsProtected).boolean().not_null())
                    .col(ColumnDef::new(Message::IsForwardingAllowed).boolean().not_null())
                    .col(ColumnDef::new(Message::UpdatedAt).timestamp_with_time_zone())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Message::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Message {
    Table,
    Id,
    ChatId,
    ContentId,
    Date,
    SenderId,
    ReplyToChatId,
    ReplyToMessageId,
    ReplyToTopMessageId,
    SendingState,
    IsDeleting,
    PreviousLocalId,
    Views,
    Forwards,
    IsEdited,
    EditDate,
    IsMentioned,
    GroupedId,
    IsScheduled,
    IsFromScheduled,
    IsSilent,
    IsPinned,
    IsProtected,
    IsForwardingAllowed,
    UpdatedAt,
}
