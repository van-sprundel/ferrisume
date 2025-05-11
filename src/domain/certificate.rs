use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Certificate {
    name: String,
    date: Option<String>,
    issuer: String,
    url: Option<String>,
}

impl Default for Certificate {
    fn default() -> Self {
        Self {
            name: "AWS Certified Solutions Architect".to_string(),
            date: Some("2021-03-15".to_string()),
            issuer: "Amazon Web Services".to_string(),
            url: Some("https://www.aws.amazon.com/certification/".to_string()),
        }
    }
}
