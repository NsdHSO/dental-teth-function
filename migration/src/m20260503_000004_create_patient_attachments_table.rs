use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table((Alias::new("dental"), PatientAttachments::Table))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PatientAttachments::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(PatientAttachments::PatientId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(PatientAttachments::Kind).text().not_null())
                    .col(ColumnDef::new(PatientAttachments::StorageUrl).text().not_null())
                    .col(ColumnDef::new(PatientAttachments::MimeType).text().not_null())
                    .col(ColumnDef::new(PatientAttachments::SizeBytes).big_integer().not_null())
                    .col(ColumnDef::new(PatientAttachments::OriginalFilename).text())
                    .col(ColumnDef::new(PatientAttachments::UploadedBy).big_integer())
                    .col(
                        ColumnDef::new(PatientAttachments::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_patient_attachments_patient_id")
                            .from(
                                (Alias::new("dental"), PatientAttachments::Table),
                                PatientAttachments::PatientId,
                            )
                            .to((Alias::new("dental"), Alias::new("patients")), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_patient_attachments_uploaded_by")
                            .from(
                                (Alias::new("dental"), PatientAttachments::Table),
                                PatientAttachments::UploadedBy,
                            )
                            .to((Alias::new("dental"), Alias::new("users")), Alias::new("id"))
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        // CHECK constraint on kind — SeaORM builder cannot express enum CHECK.
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                ALTER TABLE dental.patient_attachments
                ADD CONSTRAINT chk_patient_attachments_kind
                CHECK (kind IN ('xray','intraoral','document','pdf','other'))
                "#,
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_patient_attachments_patient_id")
                    .table((Alias::new("dental"), PatientAttachments::Table))
                    .col(PatientAttachments::PatientId)
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
                    .table((Alias::new("dental"), PatientAttachments::Table))
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum PatientAttachments {
    Table,
    Id,
    PatientId,
    Kind,
    StorageUrl,
    MimeType,
    SizeBytes,
    OriginalFilename,
    UploadedBy,
    CreatedAt,
}
