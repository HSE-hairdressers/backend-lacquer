use super::{hdresser::Hairdresser, photo::Photo};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct UserImageResponse {
    pub hairdresser: Hairdresser,
    pub images: Vec<Photo>,
    result: String,
    hairstyle: String,
}

impl UserImageResponse {
    pub fn new(h_name: Hairdresser, images: Vec<Photo>, hairstyle: &str) -> Self {
        UserImageResponse {
            hairdresser: (h_name),
            images: (images),
            result: ("Ok".to_string()),
            hairstyle: (hairstyle.to_string()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageSize {
    width: u16,
    height: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HairClassifierResponse {
    pub message: String,
    pub size: ImageSize,
    pub result: String,
}
