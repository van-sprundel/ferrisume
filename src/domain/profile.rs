use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    username: String,
    url: String,
    network: String,
}

impl Default for Profile {
    fn default() -> Self {
        Self {
            username: "johndoe".to_string(),
            url: "https://github.com/johndoe".to_string(),
            network: "GitHub".to_string(),
        }
    }
}
