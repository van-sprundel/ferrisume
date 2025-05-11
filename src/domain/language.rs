use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Language {
    language: String,
    fluency: String,
}

impl Default for Language {
    fn default() -> Self {
        Self {
            language: "English".to_string(),
            fluency: "Native".to_string(),
        }
    }
}
