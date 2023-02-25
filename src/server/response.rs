use serde::Serialize;
use super::{hdresser::Hairdresser, photo::Photo};

#[derive(Serialize)]
struct UserImageResponse {
    hairdresser: Hairdresser,
    images: Vec<Photo>,
    result: String,
}

impl UserImageResponse {
    fn new(h_name: Hairdresser, images: Vec<Photo>) -> Self {
        UserImageResponse {
            hairdresser: (h_name),
            images: (images),
            result: ("Ok".to_string()),
        }
    }
}

