use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "dental", table_name = "appointment_attachments")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub appointment_id: i64,
    pub filename: Option<String>,
    pub mime_type: String,
    pub file_data: Vec<u8>,
    pub size_bytes: i64,
    pub uploaded_by: Option<i64>,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::appointment::Entity",
        from = "Column::AppointmentId",
        to = "super::appointment::Column::Id",
        on_delete = "Cascade"
    )]
    Appointment,
}

impl Related<super::appointment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Appointment.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
