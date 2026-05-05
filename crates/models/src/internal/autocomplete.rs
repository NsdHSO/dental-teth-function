use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct AutocompleteQuery {
    pub q: String,
    #[serde(default = "default_limit")]
    pub limit: i32,
}

fn default_limit() -> i32 { 10 }

#[derive(Debug, Serialize)]
pub struct PatientAutocompleteItem {
    pub patient_id: i64,
    pub user_id: i64,
    pub full_name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub cnp: Option<String>,
    pub score: f32,
}

#[derive(Debug, Serialize)]
pub struct DentistAutocompleteItem {
    pub dentist_id: i64,
    pub user_id: i64,
    pub full_name: Option<String>,
    pub specialty: Option<String>,
    pub is_available: bool,
    pub score: f32,
}
