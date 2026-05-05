use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table((Alias::new("dental"), AppointmentAttachments::Table))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AppointmentAttachments::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(AppointmentAttachments::AppointmentId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(AppointmentAttachments::Filename).text())
                    .col(ColumnDef::new(AppointmentAttachments::MimeType).text().not_null())
                    .col(
                        ColumnDef::new(AppointmentAttachments::FileData)
                            .binary()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AppointmentAttachments::SizeBytes)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(AppointmentAttachments::UploadedBy).big_integer())
                    .col(
                        ColumnDef::new(AppointmentAttachments::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_appointment_attachments_appointment_id")
                            .from(
                                (Alias::new("dental"), AppointmentAttachments::Table),
                                AppointmentAttachments::AppointmentId,
                            )
                            .to(
                                (Alias::new("dental"), Alias::new("appointments")),
                                Alias::new("id"),
                            )
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_appointment_attachments_uploaded_by")
                            .from(
                                (Alias::new("dental"), AppointmentAttachments::Table),
                                AppointmentAttachments::UploadedBy,
                            )
                            .to((Alias::new("dental"), Alias::new("users")), Alias::new("id"))
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_appointment_attachments_appointment_id")
                    .table((Alias::new("dental"), AppointmentAttachments::Table))
                    .col(AppointmentAttachments::AppointmentId)
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table((Alias::new("dental"), AppointmentAttachments::Table))
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum AppointmentAttachments {
    Table,
    Id,
    AppointmentId,
    Filename,
    MimeType,
    FileData,
    SizeBytes,
    UploadedBy,
    CreatedAt,
}
