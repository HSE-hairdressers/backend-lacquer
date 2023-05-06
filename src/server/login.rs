use serde::{Deserialize, Serialize};

/// Login fields that used for user's sign in attempt.
#[derive(Serialize, Deserialize, Debug)]
pub struct LoginData {
    pub username: String,
    pub password: String,
}
