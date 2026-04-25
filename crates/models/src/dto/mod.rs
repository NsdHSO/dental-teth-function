pub mod role;
pub mod user;
pub mod user_role;

pub use role::{ActiveModel as RoleActiveModel, Entity as Role, Model as RoleModel};
pub use user::{ActiveModel as UserActiveModel, Entity as User, Model as UserModel};
pub use user_role::{
    ActiveModel as UserRoleActiveModel, Entity as UserRole, Model as UserRoleModel,
};