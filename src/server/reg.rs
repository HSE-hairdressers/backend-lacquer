use serde::{Deserialize, Serialize};

/// If hairdresser wants to become a member of our family
/// then he has to share following information.
#[derive(Serialize, Deserialize, Debug)]
pub struct RegistrationData {
    pub username: String,
    pub name: String,
    pub phone: String,
    pub address: String,
    pub company: String,
    pub password: String,
}
