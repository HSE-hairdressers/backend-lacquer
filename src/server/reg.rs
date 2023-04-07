use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RegistrationData {
    pub username: String,
    pub name: String,
    pub phone: String,
    pub address: String,
    pub company: String,
    pub password: String,
}
