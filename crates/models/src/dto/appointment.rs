use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "dental", table_name = "appointments")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub patient_id: i64,
    pub dentist_id: i64,
    pub appointment_date: Date,
    pub appointment_time: Time,
    pub duration: i32,
    pub status: String,
    pub reason: Option<String>,
    pub notes: Option<String>,
    pub created_by: Option<i64>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::patient::Entity",
        from = "Column::PatientId",
        to = "super::patient::Column::Id"
    )]
    Patient,

    #[sea_orm(
        belongs_to = "super::dentist::Entity",
        from = "Column::DentistId",
        to = "super::dentist::Column::Id"
    )]
    Dentist,
}

impl Related<super::patient::Entity> for Entity {
    fn to() -> RelationDef { Relation::Patient.def() }
}
impl Related<super::dentist::Entity> for Entity {
    fn to() -> RelationDef { Relation::Dentist.def() }
}

impl ActiveModelBehavior for ActiveModel {}
