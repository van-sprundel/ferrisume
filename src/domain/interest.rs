use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Interest {
    name: String,
    keywords: Option<Vec<String>>,
}

impl Default for Interest {
    fn default() -> Self {
        Self {
            name: "Programming".to_string(),
            keywords: Some(vec![
                "Open Source".to_string(),
                "Rust".to_string(),
                "Systems Programming".to_string(),
            ]),
        }
    }
}
