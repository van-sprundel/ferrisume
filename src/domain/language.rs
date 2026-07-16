use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Language {
    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fluency: Option<String>,
}

impl Default for Language {
    fn default() -> Self {
        Self {
            language: Some("English".to_string()),
            fluency: Some("Native".to_string()),
        }
    }
}
