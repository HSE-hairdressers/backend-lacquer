use serde::Serialize;

#[derive(Serialize)]
pub struct Photo {
    pub img_path: String,
}

impl Photo {
    pub fn new(path: String) -> Self {
        Photo { img_path: (path) }
    }
    pub fn from_vec(paths: Vec<String>) -> Vec<Self> {
        let mut ans: Vec<Self> = Vec::with_capacity(paths.capacity());
        for path in paths {
            ans.push(Self::new(path));
        }
        ans
    }
}
