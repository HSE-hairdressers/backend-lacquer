use super::{hdresser::Hairdresser, photo::Photo};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct UserImageResponse {
    pub data: Vec<DataResponse>,
    pub result: String,
}

impl UserImageResponse {
    pub fn new(result: &str) -> Self {
        UserImageResponse {
            data: Vec::new(),
            result: (result.to_string()),
        }
    }

    pub fn add_data(&mut self, data: DataResponse) {
        self.data.push(data);
    }
}

#[derive(Serialize)]
pub struct DataResponse {
    pub hairdresser: Hairdresser,
    pub images: Vec<Photo>,
}

impl DataResponse {
    pub fn new(h_name: Hairdresser, images: Vec<Photo>) -> Self {
        DataResponse {
            hairdresser: (h_name),
            images: (images),
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
