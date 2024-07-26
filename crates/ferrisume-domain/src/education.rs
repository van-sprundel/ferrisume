use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Education {
    #[serde(rename = "startDate")]
    start_date: String,
    #[serde(rename = "endDate")]
    end_date: Option<String>,
    area: String,
    #[serde(rename = "studyType")]
    study_type: String,
    institution: String,
    url: Option<String>,
    score: Option<String>,
    courses: Option<Vec<String>>,
}
