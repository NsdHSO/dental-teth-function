use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table((Alias::new("dental"), Appointments::Table))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Appointments::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Appointments::PatientName).string().not_null())
                    .col(ColumnDef::new(Appointments::PatientPhone).string())
                    .col(ColumnDef::new(Appointments::PatientEmail).string())
                    .col(ColumnDef::new(Appointments::DentistId).big_integer().not_null())
                    .col(ColumnDef::new(Appointments::AppointmentDate).date().not_null())
                    .col(ColumnDef::new(Appointments::AppointmentTime).time().not_null())
                    .col(
                        ColumnDef::new(Appointments::Duration)
                            .integer()
                            .not_null()
                            .default(30),
                    )
                    .col(
                        ColumnDef::new(Appointments::Status)
                            .string()
                            .not_null()
                            .default("scheduled"),
                    )
                    .col(ColumnDef::new(Appointments::Reason).text())
                    .col(ColumnDef::new(Appointments::Notes).text())
                    .col(ColumnDef::new(Appointments::CreatedBy).big_integer())
                    .col(
                        ColumnDef::new(Appointments::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Appointments::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_appointments_dentist_id")
                            .from((Alias::new("dental"), Appointments::Table), Appointments::DentistId)
                            .to((Alias::new("dental"), Alias::new("dentists")), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_appointments_date")
                    .table((Alias::new("dental"), Appointments::Table))
                    .col(Appointments::AppointmentDate)
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_appointments_dentist")
                    .table((Alias::new("dental"), Appointments::Table))
                    .col(Appointments::DentistId)
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_appointments_status")
                    .table((Alias::new("dental"), Appointments::Table))
                    .col(Appointments::Status)
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
                    .table((Alias::new("dental"), Appointments::Table))
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Appointments {
    Table,
    Id,
    PatientName,
    PatientPhone,
    PatientEmail,
    DentistId,
    AppointmentDate,
    AppointmentTime,
    Duration,
    Status,
    Reason,
    Notes,
    CreatedBy,
    CreatedAt,
    UpdatedAt,
}