use serde::{Deserialize, Serialize};

pub mod appointment;
pub mod appointment_attachment;
pub mod autocomplete;
pub mod dentist;
pub mod patient;
pub mod patient_attachment;
pub mod patient_billing;
pub mod role;
pub mod user;
pub mod user_role;
pub mod user_profile;

pub use appointment::*;
pub use appointment_attachment::*;
pub use autocomplete::*;
pub use dentist::*;
pub use patient::*;
pub use patient_attachment::*;
pub use patient_billing::*;
pub use role::*;
pub use user::*;
pub use user_role::*;
pub use user_profile::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pagination {
    pub page: i64,
    pub limit: i64,
    pub total: i64,
    pub total_pages: i64,
}
