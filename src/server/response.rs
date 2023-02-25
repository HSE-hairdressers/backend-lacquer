use super::{hdresser::Hairdresser, photo::Photo};
use serde::Serialize;

#[derive(Serialize)]
pub struct UserImageResponse {
    pub hairdresser: Hairdresser,
    pub images: Vec<Photo>,
    result: String,
}

impl UserImageResponse {
    pub fn new(h_name: Hairdresser, images: Vec<Photo>) -> Self {
        UserImageResponse {
            hairdresser: (h_name),
            images: (images),
            result: ("Ok".to_string()),
        }
    }
}
