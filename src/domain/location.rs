use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    #[serde(skip_serializing_if = "Option::is_none")]
    city: Option<String>,
    #[serde(rename = "countryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    country_code: Option<String>,
}

impl Default for Location {
    fn default() -> Self {
        Self {
            city: Some("San Francisco".to_string()),
            country_code: Some("US".to_string()),
        }
    }
}
