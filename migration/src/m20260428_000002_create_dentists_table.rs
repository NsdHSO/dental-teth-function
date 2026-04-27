use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table((Alias::new("dental"), Dentists::Table))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Dentists::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Dentists::UserId)
                            .big_integer()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Dentists::Specialty).string())
                    .col(ColumnDef::new(Dentists::LicenseNumber).string())
                    .col(ColumnDef::new(Dentists::PhotoUrl).text())
                    .col(ColumnDef::new(Dentists::Bio).text())
                    .col(
                        ColumnDef::new(Dentists::IsAvailable)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(Dentists::ConsultationDuration)
                            .integer()
                            .not_null()
                            .default(30),
                    )
                    .col(
                        ColumnDef::new(Dentists::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Dentists::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_dentists_user_id")
                            .from((Alias::new("dental"), Dentists::Table), Dentists::UserId)
                            .to((Alias::new("dental"), Alias::new("users")), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_dentists_specialty")
                    .table((Alias::new("dental"), Dentists::Table))
                    .col(Dentists::Specialty)
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
                    .table((Alias::new("dental"), Dentists::Table))
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Dentists {
    Table,
    Id,
    UserId,
    Specialty,
    LicenseNumber,
    PhotoUrl,
    Bio,
    IsAvailable,
    ConsultationDuration,
    CreatedAt,
    UpdatedAt,
}