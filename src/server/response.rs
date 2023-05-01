use super::{hdresser::Hairdresser, photo::Photo};
use serde::{Deserialize, Serialize};

/* main response on user Image */
#[derive(Serialize)]
pub struct HairdresserResponse {
    data: Vec<HairdresserData>,
}

impl HairdresserResponse {
    pub fn builder() -> HairdresserResponseBuilder {
        HairdresserResponseBuilder::default()
    }
}

#[derive(Default)]
pub struct HairdresserResponseBuilder {
    data: Vec<HairdresserData>,
}

impl HairdresserResponseBuilder {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn add_data(&mut self, data: HairdresserData) {
        self.data.push(data);
    }

    pub fn with_data(data: Vec<HairdresserData>) -> Self {
        Self { data: (data) }
    }

    pub fn build(self) -> HairdresserResponse {
        HairdresserResponse { data: (self.data) }
    }
}

/* Every single hairdresser with their photos */
#[derive(Serialize)]
pub struct HairdresserData {
    pub hairdresser: Hairdresser,
    pub images: Vec<Photo>,
}

impl HairdresserData {
    pub fn new(h_name: Hairdresser, images: Vec<Photo>) -> Self {
        HairdresserData {
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
        if self.result != "0" {
            Some(self.result.to_string())
        } else {
            None
        }
    }
}
