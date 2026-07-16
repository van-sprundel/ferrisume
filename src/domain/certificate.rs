use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Certificate {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    issuer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
}

impl Default for Certificate {
    fn default() -> Self {
        Self {
            name: Some("AWS Certified Solutions Architect".to_string()),
            date: Some("2021-03-15".to_string()),
            issuer: Some("Amazon Web Services".to_string()),
            url: Some("https://www.aws.amazon.com/certification/".to_string()),
        }
    }
}
