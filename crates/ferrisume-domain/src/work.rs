use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Work {
    name: String,
    position: String,
    url: String,
    #[serde(rename = "startDate")]
    start_date: String,
    #[serde(rename = "endDate")]
    end_date: Option<String>,
    summary: String,
    highlights: Option<Vec<String>>,
}
