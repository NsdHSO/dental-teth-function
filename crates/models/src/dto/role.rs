use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "church", table_name = "roles")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,

    #[sea_orm(unique)]
    pub name: String, // Pastor, Elder, Deacon, Cell Leader, Member, Admin, etc.
    pub description: Option<String>,
    pub level: i32, // Hierarchy level: 1=Member, 2=Cell Leader, 3=Zone Leader, 4=Pastor, 5=Admin
    pub permissions: Option<String>, // JSON array of permission strings
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
