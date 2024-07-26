use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Skill {
    keywords: Vec<String>,
    level: String,
    name: String,
}
