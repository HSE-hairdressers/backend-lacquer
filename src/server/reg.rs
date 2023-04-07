use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RegistrationData {
    username: String,
    name: String,
    phone: String,
    address: String,
    company: String,
    password: String,
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
