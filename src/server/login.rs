use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginData {
    pub username: String,
    pub password: String,
}
