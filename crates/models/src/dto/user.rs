use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "dental", table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,

    #[sea_orm(unique)]
    pub auth_user_id: String,

    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::user_role::Entity")]
    UserRoles,
}

impl Related<super::user_role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserRoles.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}