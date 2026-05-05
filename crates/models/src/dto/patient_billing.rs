use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "dental", table_name = "patient_billings")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub patient_id: i64,
    pub appointment_id: Option<i64>,
    pub amount_cents: i64,
    pub currency: String,
    pub status: String,
    pub description: Option<String>,
    pub paid_at: Option<DateTime>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::patient::Entity",
        from = "Column::PatientId",
        to = "super::patient::Column::Id",
        on_delete = "Restrict"
    )]
    Patient,

    #[sea_orm(
        belongs_to = "super::appointment::Entity",
        from = "Column::AppointmentId",
        to = "super::appointment::Column::Id",
        on_delete = "SetNull"
    )]
    Appointment,
}

impl Related<super::patient::Entity>     for Entity { fn to() -> RelationDef { Relation::Patient.def() } }
impl Related<super::appointment::Entity> for Entity { fn to() -> RelationDef { Relation::Appointment.def() } }

impl ActiveModelBehavior for ActiveModel {}
