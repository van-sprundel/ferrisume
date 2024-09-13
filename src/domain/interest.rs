use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Interest {
    name: String,
    keywords: Option<Vec<String>>,
}
