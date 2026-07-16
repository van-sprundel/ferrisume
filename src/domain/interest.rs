use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Interest {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    keywords: Option<Vec<String>>,
}

impl Default for Interest {
    fn default() -> Self {
        Self {
            name: Some("Programming".to_string()),
            keywords: Some(vec![
                "Open Source".to_string(),
                "Rust".to_string(),
                "Systems Programming".to_string(),
            ]),
        }
    }
}
