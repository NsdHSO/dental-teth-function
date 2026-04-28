pub mod appointment;
pub mod dentist;
pub mod role;
pub mod user;
pub mod user_role;
pub mod user_profile;

pub use appointment::{ActiveModel as AppointmentActiveModel, Entity as Appointment, Model as AppointmentModel};
pub use dentist::{ActiveModel as DentistActiveModel, Entity as Dentist, Model as DentistModel};
pub use role::{ActiveModel as RoleActiveModel, Entity as Role, Model as RoleModel};
pub use user::{ActiveModel as UserActiveModel, Entity as User, Model as UserModel};
pub use user_role::{
    ActiveModel as UserRoleActiveModel, Entity as UserRole, Model as UserRoleModel,
};
pub use user_profile::{ActiveModel as UserProfileActiveModel, Entity as UserProfile, Model as UserProfileModel};