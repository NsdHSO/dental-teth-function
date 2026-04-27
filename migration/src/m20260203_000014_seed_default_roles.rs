use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let insert = Query::insert()
            .into_table((Alias::new("dental"), Roles::Table))
            .columns([
                Roles::Name,
                Roles::Description,
                Roles::Level,
                Roles::Permissions,
            ])
            .values_panic([
                "Patient".into(),
                "Regular patient with basic access to view own appointments".into(),
                1.into(),
                r#"["view_profile","update_own_profile","view_own_appointments","book_appointment"]"#.into(),
            ])
            .values_panic([
                "Dentist".into(),
                "Licensed dentist performing dental procedures".into(),
                5.into(),
                r#"["view_profile","update_own_profile","view_appointments","manage_patients","view_medical_records","create_prescription","view_reports"]"#.into(),
            ])
            .values_panic([
                "Receptionist".into(),
                "Front desk staff managing appointments and patient check-in".into(),
                3.into(),
                r#"["view_profile","update_own_profile","manage_appointments","register_patient","view_patients","update_patient_info","view_schedule"]"#.into(),
            ])
            .values_panic([
                "Admin".into(),
                "System administrator with full access to all features".into(),
                10.into(),
                r#"["all"]"#.into(),
            ])
            .values_panic([
                "Dental Assistant".into(),
                "Assists dentist during procedures and manages equipment".into(),
                4.into(),
                r#"["view_profile","update_own_profile","view_appointments","assist_dentist","manage_equipment","view_patients"]"#.into(),
            ])
            .values_panic([
                "Dental Hygienist".into(),
                "Performs dental cleanings and patient education".into(),
                5.into(),
                r#"["view_profile","update_own_profile","view_appointments","perform_cleaning","patient_education","view_medical_records"]"#.into(),
            ])
            .values_panic([
                "Orthodontist".into(),
                "Specializes in teeth alignment and braces".into(),
                6.into(),
                r#"["view_profile","update_own_profile","view_appointments","manage_orthodontic","create_treatment_plan","view_medical_records","imaging"]"#.into(),
            ])
            .values_panic([
                "Oral Surgeon".into(),
                "Performs surgical procedures like extractions and implants".into(),
                6.into(),
                r#"["view_profile","update_own_profile","view_appointments","perform_surgery","create_treatment_plan","view_medical_records","prescribe_medication"]"#.into(),
            ])
            .values_panic([
                "Pediatric Dentist".into(),
                "Specializes in dental care for children".into(),
                6.into(),
                r#"["view_profile","update_own_profile","view_appointments","manage_pediatric","view_medical_records","patient_education","communicate_parent"]"#.into(),
            ])
            .values_panic([
                "Periodontist".into(),
                "Treats gum disease and dental implants".into(),
                6.into(),
                r#"["view_profile","update_own_profile","view_appointments","treat_gum_disease","place_implants","view_medical_records"]"#.into(),
            ])
            .values_panic([
                "Endodontist".into(),
                "Root canal specialist".into(),
                6.into(),
                r#"["view_profile","update_own_profile","view_appointments","perform_root_canal","view_medical_records","create_treatment_plan"]"#.into(),
            ])
            .values_panic([
                "Dental Technician".into(),
                "Creates dental prosthetics like crowns and dentures".into(),
                4.into(),
                r#"["view_profile","update_own_profile","view_orders","create_prosthetics","lab_management","quality_control"]"#.into(),
            ])
            .values_panic([
                "Clinic Manager".into(),
                "Oversees clinic operations and staff management".into(),
                7.into(),
                r#"["view_profile","update_own_profile","manage_staff","view_reports","manage_schedule","inventory","financial_overview","settings"]"#.into(),
            ])
            .values_panic([
                "Insurance Coordinator".into(),
                "Handles insurance claims and billing".into(),
                4.into(),
                r#"["view_profile","update_own_profile","view_patients","process_claims","billing","view_insurance_info","communicate_insurance"]"#.into(),
            ])
            .values_panic([
                "Dental Consultant".into(),
                "Advisory and strategy role for clinic improvement".into(),
                7.into(),
                r#"["view_profile","update_own_profile","view_reports","consult_strategy","analyze_operations","staff_training","quality_audit"]"#.into(),
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
        let delete = Query::delete()
            .from_table((Alias::new("dental"), Roles::Table))
            .and_where(Expr::col(Roles::Name).is_in([
                "Patient",
                "Dentist",
                "Receptionist",
                "Admin",
                "Dental Assistant",
                "Dental Hygienist",
                "Orthodontist",
                "Oral Surgeon",
                "Pediatric Dentist",
                "Periodontist",
                "Endodontist",
                "Dental Technician",
                "Clinic Manager",
                "Insurance Coordinator",
                "Dental Consultant",
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