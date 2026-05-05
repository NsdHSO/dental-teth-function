pub mod dto;
pub mod internal;

pub use dto::{
    Appointment, AppointmentActiveModel, AppointmentModel,
    AppointmentAttachment, AppointmentAttachmentActiveModel, AppointmentAttachmentModel,
    Dentist, DentistActiveModel, DentistModel,
    Patient, PatientActiveModel, PatientModel,
    PatientAttachment, PatientAttachmentActiveModel, PatientAttachmentModel,
    PatientBilling, PatientBillingActiveModel, PatientBillingModel,
    Role, RoleActiveModel, RoleModel,
    User, UserActiveModel, UserModel,
    UserProfile, UserProfileActiveModel, UserProfileModel,
    UserRole, UserRoleActiveModel, UserRoleModel,
};
