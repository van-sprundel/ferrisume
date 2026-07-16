use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<String>,
}

impl Default for Profile {
    fn default() -> Self {
        Self {
            username: Some("johndoe".to_string()),
            url: Some("https://github.com/johndoe".to_string()),
            network: Some("GitHub".to_string()),
        }
    }
}
