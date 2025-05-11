use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    city: String,
    #[serde(rename = "countryCode")]
    country_code: String,
}

impl Default for Location {
    fn default() -> Self {
        Self {
            city: "San Francisco".to_string(),
            country_code: "US".to_string(),
        }
    }
}
