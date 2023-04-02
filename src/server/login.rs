use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoginData {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub result: String,
    pub response: String,
}

impl LoginResponse {
    pub fn new(res: &str, resp: &str) -> Self {
        Self {
            result: res.to_string(),
            response: resp.to_string(),
        }
    }
}
