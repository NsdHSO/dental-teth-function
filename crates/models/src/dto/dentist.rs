use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "dental", table_name = "dentists")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub user_id: i64,
    pub specialty: Option<String>,
    pub license_number: Option<String>,
    pub photo_url: Option<String>,
    pub bio: Option<String>,
    pub is_available: bool,
    pub consultation_duration: i32,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::appointment::Entity")]
    Appointments,
}

impl Related<super::appointment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Appointments.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}