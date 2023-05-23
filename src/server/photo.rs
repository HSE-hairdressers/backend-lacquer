use serde::{Deserialize, Serialize};

/// All humans loves photos.
#[derive(Serialize, Deserialize, Debug)]
pub struct Photo {
    /// URL of the photo.
    pub img_path: String,
}

impl Photo {
    /// Returns a photo object with the given photo's URL.
    ///
    /// # Arguments
    ///
    /// * `path` - A string that holds photo's URL.
    pub fn new(path: String) -> Self {
        Photo { img_path: (path) }
    }
    /// Returns a vector with photo objects with the given photo's URLs.
    ///
    /// # Arguments
    ///
    /// * `paths` - A vector with strings that hold photo's URLs.
    pub fn from_vec(paths: Vec<String>) -> Vec<Self> {
        let mut ans: Vec<Self> = Vec::with_capacity(paths.capacity());
        for path in paths {
            ans.push(Self::new(path));
        }
        ans
    }
}
