use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Certificate {
    name: String,
    date: Option<String>,
    issuer: String,
    url: Option<String>,
}
