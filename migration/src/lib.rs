pub use sea_orm_migration::prelude::*;

mod m20260428_000001_create_dental_schema;
mod m20260203_000002_create_users_table;
mod m20260203_000007_create_roles_table;
mod m20260203_000010_create_user_roles_table;
mod m20260203_000014_seed_default_roles;
mod m20260428_000002_create_dentists_table;
mod m20260428_000003_create_appointments_table;
mod m20260428_000006_create_user_profiles_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260428_000001_create_dental_schema::Migration),
            Box::new(m20260203_000002_create_users_table::Migration),
            Box::new(m20260203_000007_create_roles_table::Migration),
            Box::new(m20260203_000010_create_user_roles_table::Migration),
            Box::new(m20260203_000014_seed_default_roles::Migration),
            Box::new(m20260428_000002_create_dentists_table::Migration),
            Box::new(m20260428_000003_create_appointments_table::Migration),
            Box::new(m20260428_000006_create_user_profiles_table::Migration),
        ]
    }
}