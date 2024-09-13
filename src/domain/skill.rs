use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Skill {
    keywords: Vec<String>,
    level: String,
    name: String,
}
