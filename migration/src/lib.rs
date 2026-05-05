pub use sea_orm_migration::prelude::*;

mod m20260428_000001_create_dental_schema;
mod m20260203_000002_create_users_table;
mod m20260203_000007_create_roles_table;
mod m20260203_000010_create_user_roles_table;
mod m20260203_000014_seed_default_roles;
mod m20260428_000002_create_dentists_table;
mod m20260428_000003_create_appointments_table;
mod m20260428_000006_create_user_profiles_table;
mod m20260503_000001_enable_autocomplete_extensions;
mod m20260503_000002_extend_user_profiles_identity;
mod m20260503_000003_create_patients_table;
mod m20260503_000004_create_patient_attachments_table;
mod m20260503_000005_create_patient_billings_table;
mod m20260503_000006_appointments_link_patient;
mod m20260503_000007_create_autocomplete_functions;
mod m20260504_000001_create_appointment_attachments_table;
mod m20260505_000001_fix_f_unaccent_search_path;

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
            Box::new(m20260503_000001_enable_autocomplete_extensions::Migration),
            Box::new(m20260503_000002_extend_user_profiles_identity::Migration),
            Box::new(m20260503_000003_create_patients_table::Migration),
            Box::new(m20260503_000004_create_patient_attachments_table::Migration),
            Box::new(m20260503_000005_create_patient_billings_table::Migration),
            Box::new(m20260503_000006_appointments_link_patient::Migration),
            Box::new(m20260503_000007_create_autocomplete_functions::Migration),
            Box::new(m20260504_000001_create_appointment_attachments_table::Migration),
            Box::new(m20260505_000001_fix_f_unaccent_search_path::Migration),
        ]
    }
}
