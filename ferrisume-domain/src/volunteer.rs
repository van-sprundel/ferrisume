use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Volunteer {
    organization: String,
    position: String,
    url: Option<String>,
    #[serde(rename = "startDate")]
    start_date: Option<String>,
    #[serde(rename = "endDate")]
    end_date: Option<String>,
    summary: Option<String>,
    highlights: Option<Vec<String>>,
}
