use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "dental", table_name = "appointments")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub patient_name: String,
    pub patient_phone: Option<String>,
    pub patient_email: Option<String>,
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
    #[sea_orm(has_one = "super::dentist::Entity")]
    Dentist,
}

impl Related<super::dentist::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Dentist.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}