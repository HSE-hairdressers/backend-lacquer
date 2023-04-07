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

#[derive(Serialize, Deserialize, Debug)]
pub struct RegistrationResponse {
    pub result: String,
}

impl RegistrationResponse {
    pub fn new(res: &str) -> Self {
        Self {
            result: res.to_string(),
        }
    }
}
