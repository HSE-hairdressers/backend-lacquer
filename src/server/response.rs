use super::{hdresser::Hairdresser, photo::Photo};
use serde::{Deserialize, Serialize};

/* main response on user Image */
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

/* Every single hairdresser with their photos */
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

impl HairClassifierResponse {
    pub fn get_result(&self) -> Option<String> {
        if self.result != "Error" {
            Some(self.result.to_string())
        } else {
            None
        }
    }
}
