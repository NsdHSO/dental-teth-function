use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Build the insert query with ON CONFLICT support
        let insert = Query::insert()
            .into_table((Alias::new("church"), Roles::Table))
            .columns([
                Roles::Name,
                Roles::Description,
                Roles::Level,
                Roles::Permissions,
            ])
            .values_panic([
                "Member".into(),
                "Regular church member with basic access".into(),
                1.into(),
                r#"["view_profile","update_own_profile","view_events"]"#.into(),
            ])
            .values_panic([
                "Cell Leader".into(),
                "Cell group leader responsible for a small group".into(),
                2.into(),
                r#"["view_profile","update_own_profile","view_events","manage_cell_group","view_cell_members","record_cell_attendance"]"#.into(),
            ])
            .values_panic([
                "Zone Leader".into(),
                "Zone leader overseeing multiple cell groups".into(),
                3.into(),
                r#"["view_profile","update_own_profile","view_events","manage_zone","view_zone_cells","view_zone_members","record_zone_attendance"]"#.into(),
            ])
            .values_panic([
                "Deacon".into(),
                "Deacon serving in church ministry".into(),
                3.into(),
                r#"["view_profile","update_own_profile","view_events","manage_ministry","view_members","record_attendance"]"#.into(),
            ])
            .values_panic([
                "Elder".into(),
                "Church elder with leadership responsibilities".into(),
                4.into(),
                r#"["view_profile","update_own_profile","view_events","manage_ministries","view_all_members","record_attendance","manage_roles","view_reports"]"#.into(),
            ])
            .values_panic([
                "Pastor".into(),
                "Senior pastor with full church oversight".into(),
                4.into(),
                r#"["view_profile","update_own_profile","view_events","manage_church","view_all_members","manage_all_ministries","manage_all_roles","view_all_reports","manage_financials"]"#.into(),
            ])
            .values_panic([
                "Admin".into(),
                "System administrator with full access".into(),
                5.into(),
                r#"["all"]"#.into(),
            ])
            .on_conflict(
                OnConflict::column(Roles::Name)
                    .do_nothing()
                    .to_owned()
            )
            .to_owned();

        manager.exec_stmt(insert).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Delete all default roles
        let delete = Query::delete()
            .from_table((Alias::new("church"), Roles::Table))
            .and_where(Expr::col(Roles::Name).is_in([
                "Member",
                "Cell Leader",
                "Zone Leader",
                "Deacon",
                "Elder",
                "Pastor",
                "Admin",
            ]))
            .to_owned();

        manager.exec_stmt(delete).await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Roles {
    Table,
    Name,
    Description,
    Level,
    Permissions,
}
