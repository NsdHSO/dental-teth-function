pub mod appointments;
pub mod bootstrap;
pub mod dentists;
pub mod roles;
pub mod user_roles;
pub mod users;

pub use appointments::configure_appointments;
pub use bootstrap::configure_bootstrap;
pub use dentists::configure_dentists;
pub use roles::configure_roles;
pub use user_roles::configure_user_roles;
pub use users::configure_users;