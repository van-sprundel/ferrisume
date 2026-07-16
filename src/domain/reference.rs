use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Reference {
    #[serde(skip_serializing_if = "Option::is_none")]
    reference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

impl Default for Reference {
    fn default() -> Self {
        Self {
            reference: Some("John is an exceptional developer with strong problem-solving skills and attention to detail.".to_string()),
            name: Some("Jane Smith, Engineering Manager".to_string()),
        }
    }
}
