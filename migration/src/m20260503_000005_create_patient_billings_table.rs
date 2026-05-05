use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table((Alias::new("dental"), PatientBillings::Table))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PatientBillings::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(PatientBillings::PatientId).big_integer().not_null())
                    .col(ColumnDef::new(PatientBillings::AppointmentId).big_integer())
                    .col(ColumnDef::new(PatientBillings::AmountCents).big_integer().not_null())
                    .col(
                        ColumnDef::new(PatientBillings::Currency)
                            .text()
                            .not_null()
                            .default("RON"),
                    )
                    .col(
                        ColumnDef::new(PatientBillings::Status)
                            .text()
                            .not_null()
                            .default("draft"),
                    )
                    .col(ColumnDef::new(PatientBillings::Description).text())
                    .col(ColumnDef::new(PatientBillings::PaidAt).timestamp())
                    .col(
                        ColumnDef::new(PatientBillings::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(PatientBillings::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_patient_billings_patient_id")
                            .from(
                                (Alias::new("dental"), PatientBillings::Table),
                                PatientBillings::PatientId,
                            )
                            .to((Alias::new("dental"), Alias::new("patients")), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_patient_billings_appointment_id")
                            .from(
                                (Alias::new("dental"), PatientBillings::Table),
                                PatientBillings::AppointmentId,
                            )
                            .to(
                                (Alias::new("dental"), Alias::new("appointments")),
                                Alias::new("id"),
                            )
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                ALTER TABLE dental.patient_billings
                    DROP CONSTRAINT IF EXISTS chk_patient_billings_status,
                    ADD CONSTRAINT chk_patient_billings_status
                    CHECK (status IN ('draft','issued','paid','refunded','void'))
                "#,
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_patient_billings_patient_id")
                    .table((Alias::new("dental"), PatientBillings::Table))
                    .col(PatientBillings::PatientId)
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_patient_billings_status_created_at")
                    .table((Alias::new("dental"), PatientBillings::Table))
                    .col(PatientBillings::Status)
                    .col(PatientBillings::CreatedAt)
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
                    .table((Alias::new("dental"), PatientBillings::Table))
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum PatientBillings {
    Table,
    Id,
    PatientId,
    AppointmentId,
    AmountCents,
    Currency,
    Status,
    Description,
    PaidAt,
    CreatedAt,
    UpdatedAt,
}
