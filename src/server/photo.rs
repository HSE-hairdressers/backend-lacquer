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
    pub fn from_vec(paths: Vec<String>) -> Vec<Self> {
        let mut ans: Vec<Self> = Vec::with_capacity(paths.capacity());
        for path in paths {
            ans.push(Self::new(
                path.clone().split("/").last().unwrap().to_string(),
                path,
            ));
        }
        ans
    }
}
