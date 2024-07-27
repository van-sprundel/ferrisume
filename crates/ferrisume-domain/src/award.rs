use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Award {
    title: String,
    awarder: String,
    date: String,
    summary: Option<String>,
}
