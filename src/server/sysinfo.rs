use serde::Serialize;

#[derive(Serialize, Default)]
pub struct SystemInfo {
    pub name: String,
    pub version: String,
}
