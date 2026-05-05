use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "dental", table_name = "patient_attachments")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub patient_id: i64,
    pub kind: String,
    pub storage_url: String,
    pub mime_type: String,
    pub size_bytes: i64,
    pub original_filename: Option<String>,
    pub uploaded_by: Option<i64>,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::patient::Entity",
        from = "Column::PatientId",
        to = "super::patient::Column::Id",
        on_delete = "Cascade"
    )]
    Patient,
}

impl Related<super::patient::Entity> for Entity {
    fn to() -> RelationDef { Relation::Patient.def() }
}

impl ActiveModelBehavior for ActiveModel {}
