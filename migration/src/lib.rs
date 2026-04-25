pub use sea_orm_migration::prelude::*;

mod m20260203_000001_create_church_schema;
mod m20260203_000002_create_users_table;
mod m20260203_000007_create_roles_table;
mod m20260203_000010_create_user_roles_table;
mod m20260203_000014_seed_default_roles;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260203_000001_create_church_schema::Migration),
            Box::new(m20260203_000002_create_users_table::Migration),
            Box::new(m20260203_000007_create_roles_table::Migration),
            Box::new(m20260203_000010_create_user_roles_table::Migration),
            Box::new(m20260203_000014_seed_default_roles::Migration),
        ]
    }
}