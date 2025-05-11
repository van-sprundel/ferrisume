use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Reference {
    reference: String,
    name: String,
}

impl Default for Reference {
    fn default() -> Self {
        Self {
            reference: "John is an exceptional developer with strong problem-solving skills and attention to detail.".to_string(),
            name: "Jane Smith, Engineering Manager".to_string(),
        }
    }
}
