use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table((Alias::new("dental"), UserProfiles::Table))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserProfiles::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(UserProfiles::UserSub)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(UserProfiles::SchemaVersion)
                            .string()
                            .not_null()
                            .default("1.0"),
                    )
                    .col(
                        ColumnDef::new(UserProfiles::Attributes)
                            .json_binary()
                            .not_null()
                            .default(Expr::val("{}")),
                    )
                    .col(
                        ColumnDef::new(UserProfiles::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(UserProfiles::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_user_profiles_user_sub")
                    .table((Alias::new("dental"), UserProfiles::Table))
                    .col(UserProfiles::UserSub)
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
                    .table((Alias::new("dental"), UserProfiles::Table))
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum UserProfiles {
    Table,
    Id,
    UserSub,
    SchemaVersion,
    Attributes,
    CreatedAt,
    UpdatedAt,
}