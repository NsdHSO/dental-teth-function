use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table((Alias::new("church"), UserRoles::Table))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserRoles::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UserRoles::UserId).big_integer().not_null())
                    .col(ColumnDef::new(UserRoles::RoleId).big_integer().not_null())
                    .col(ColumnDef::new(UserRoles::AssignedDate).date().not_null())
                    .col(ColumnDef::new(UserRoles::AssignedBy).big_integer())
                    .col(
                        ColumnDef::new(UserRoles::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(UserRoles::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(UserRoles::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_roles_user_id")
                            .from((Alias::new("church"), UserRoles::Table), UserRoles::UserId)
                            .to(
                                (Alias::new("church"), Alias::new("users")),
                                Alias::new("id"),
                            )
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_roles_role_id")
                            .from((Alias::new("church"), UserRoles::Table), UserRoles::RoleId)
                            .to(
                                (Alias::new("church"), Alias::new("roles")),
                                Alias::new("id"),
                            )
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_roles_assigned_by")
                            .from(
                                (Alias::new("church"), UserRoles::Table),
                                UserRoles::AssignedBy,
                            )
                            .to(
                                (Alias::new("church"), Alias::new("users")),
                                Alias::new("id"),
                            )
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create unique constraint: one user can have one specific role only once
        manager
            .create_index(
                Index::create()
                    .name("idx_user_roles_user_role_unique")
                    .table((Alias::new("church"), UserRoles::Table))
                    .col(UserRoles::UserId)
                    .col(UserRoles::RoleId)
                    .unique()
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
                    .table((Alias::new("church"), UserRoles::Table))
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum UserRoles {
    Table,
    Id,
    UserId,
    RoleId,
    AssignedDate,
    AssignedBy,
    IsActive,
    CreatedAt,
    UpdatedAt,
}
