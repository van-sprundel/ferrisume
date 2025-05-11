use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Skill {
    keywords: Vec<String>,
    level: String,
    name: String,
}

impl Default for Skill {
    fn default() -> Self {
        Self {
            keywords: vec![
                "Rust".to_string(),
                "WebAssembly".to_string(),
                "Microservices".to_string(),
            ],
            level: "Advanced".to_string(),
            name: "Backend Development".to_string(),
        }
    }
}
