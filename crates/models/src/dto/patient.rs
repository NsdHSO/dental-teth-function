use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "dental", table_name = "patients")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub user_id: i64,
    pub medical_notes: Option<String>,
    pub allergies: Option<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_delete = "Cascade"
    )]
    User,

    #[sea_orm(has_many = "super::appointment::Entity")]
    Appointments,

    #[sea_orm(has_many = "super::patient_attachment::Entity")]
    Attachments,

    #[sea_orm(has_many = "super::patient_billing::Entity")]
    Billings,
}

impl Related<super::user::Entity>               for Entity { fn to() -> RelationDef { Relation::User.def() } }
impl Related<super::appointment::Entity>        for Entity { fn to() -> RelationDef { Relation::Appointments.def() } }
impl Related<super::patient_attachment::Entity> for Entity { fn to() -> RelationDef { Relation::Attachments.def() } }
impl Related<super::patient_billing::Entity>    for Entity { fn to() -> RelationDef { Relation::Billings.def() } }

impl ActiveModelBehavior for ActiveModel {}
