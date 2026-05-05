use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table((Alias::new("dental"), Patients::Table))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Patients::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Patients::UserId)
                            .big_integer()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Patients::MedicalNotes).text())
                    .col(ColumnDef::new(Patients::Allergies).text())
                    .col(
                        ColumnDef::new(Patients::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Patients::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_patients_user_id")
                            .from((Alias::new("dental"), Patients::Table), Patients::UserId)
                            .to((Alias::new("dental"), Alias::new("users")), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table((Alias::new("dental"), Patients::Table))
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Patients {
    Table,
    Id,
    UserId,
    MedicalNotes,
    Allergies,
    CreatedAt,
    UpdatedAt,
}
