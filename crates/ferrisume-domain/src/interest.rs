use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Interest {
    name: String,
    keywords: Option<Vec<String>>,
}
