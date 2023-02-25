use serde::Serialize;

#[derive(Serialize)]
pub struct Photo {
    pub name: String,
    pub binary: Vec<u8>,
}

impl Photo {
    pub fn new(name: String, binary: Vec<u8>) -> Self {
        Photo {
            name: (name),
            binary: (binary),
        }
    }
}

