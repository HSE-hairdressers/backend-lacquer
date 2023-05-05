use super::{hdresser::Hairdresser, photo::Photo};
use serde::{Deserialize, Serialize};

/// Main response used in service `/img`.
#[derive(Serialize)]
pub struct HairdresserResponse {
    /// This vector contains hairdressers with their works (photos).
    data: Vec<HairdresserData>,
}

impl HairdresserResponse {
    /// Returns a builder for main response object.
    pub fn builder() -> HairdresserResponseBuilder {
        HairdresserResponseBuilder::default()
    }
}

/// A builder for main the response.
#[derive(Default)]
pub struct HairdresserResponseBuilder {
    /// This vector contains hairdressers with their works (photos).
    data: Vec<HairdresserData>,
}

impl HairdresserResponseBuilder {
    /// Returns a builder with empty initialized vector.
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    /// Adds data to the current builder.
    ///
    /// # Arguments
    ///
    /// * `data` - single hairdresser's data object with hairdresser and its works (photos).
    pub fn add_data(&mut self, data: HairdresserData) {
        self.data.push(data);
    }

    /// Returns a builde with given vector with data.
    ///
    /// # Arguments
    ///
    /// * `data` - A vector that contains  hairdressers with its works (photos).
    pub fn with_data(data: Vec<HairdresserData>) -> Self {
        Self { data: (data) }
    }

    /// Returns an object of the main response with the built data.
    pub fn build(self) -> HairdresserResponse {
        HairdresserResponse { data: (self.data) }
    }
}

/// Every single hairdresser with their photos.
#[derive(Serialize)]
pub struct HairdresserData {
    /// A hairdresser.
    pub hairdresser: Hairdresser,
    /// current hairdresser's works (photos).
    pub images: Vec<Photo>,
}

impl HairdresserData {
    /// Returns hairdresser data response with the given data.
    ///
    /// # Arguments
    ///
    /// * `h_name` - single hairdresser object.
    /// * `images` - vector of photos.
    pub fn new(h_name: Hairdresser, images: Vec<Photo>) -> Self {
        HairdresserData {
            hairdresser: (h_name),
            images: (images),
        }
    }
}

/// Every image has its width and height size in pixels.
#[derive(Serialize, Deserialize, Debug)]
pub struct ImageSize {
    width: u16,
    height: u16,
}

/// Response that we get as a AI's work result.
#[derive(Serialize, Deserialize, Debug)]
pub struct HairClassifierResponse {
    /// message (I don't know we AI sends me message, to be honest).
    pub message: String,
    /// size of the image I sent to AI.
    pub size: ImageSize,
    /// Result of hairstyle's recognition.
    pub result: String,
}

impl HairClassifierResponse {
    /// Returns extracted result from AI's response.
    pub fn get_result(&self) -> Option<String> {
        if self.result != "0" {
            Some(self.result.to_string())
        } else {
            None
        }
    }
}
