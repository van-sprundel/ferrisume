use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    city: String,
    #[serde(rename = "countryCode")]
    country_code: String,
}
