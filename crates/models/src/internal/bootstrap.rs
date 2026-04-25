use serde::{Deserialize, Serialize};

use crate::internal::UserResponse;

#[derive(Debug, Deserialize)]
pub struct BootstrapRequest {
    #[serde(default)]
    pub create_profile_if_missing: bool,
}

#[derive(Debug, Serialize)]
pub struct BootstrapResponse {
    pub user: UserResponse,
}

#[derive(Debug, Serialize, Default)]
pub struct BootstrapCreated {
    pub linked: bool,
}