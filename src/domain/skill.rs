use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Skill {
    #[serde(skip_serializing_if = "Option::is_none")]
    keywords: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

impl Default for Skill {
    fn default() -> Self {
        Self {
            keywords: Some(vec![
                "Rust".to_string(),
                "WebAssembly".to_string(),
                "Microservices".to_string(),
            ]),
            level: Some("Advanced".to_string()),
            name: Some("Backend Development".to_string()),
        }
    }
}
