use serde::Serialize;

/// In order to identify which server's version we use
/// we only need the name of a project and its version;
#[derive(Serialize, Default)]
pub struct SystemInfo {
    pub name: String,
    pub version: String,
}
