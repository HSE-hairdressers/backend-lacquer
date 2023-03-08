use serde::Serialize;

#[derive(Serialize)]
pub struct Photo {
    pub name: String,
    pub img_path: String,
}

impl Photo {
    pub fn new(name: String, path: String) -> Self {
        Photo {
            name: (name),
            img_path: (path),
        }
    }
}
