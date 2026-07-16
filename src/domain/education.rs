use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Education {
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    start_date: Option<String>,
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    end_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    area: Option<String>,
    #[serde(rename = "studyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    study_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    institution: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    score: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    courses: Option<Vec<String>>,
}

impl Default for Education {
    fn default() -> Self {
        Self {
            start_date: Some("2014-09-01".to_string()),
            end_date: Some("2018-05-15".to_string()),
            area: Some("Computer Science".to_string()),
            study_type: Some("Bachelor of Science".to_string()),
            institution: Some("University of Technology".to_string()),
            url: Some("https://www.universityoftech.edu".to_string()),
            score: Some("3.8/4.0".to_string()),
            courses: Some(vec![
                "Data Structures and Algorithms".to_string(),
                "Operating Systems".to_string(),
                "Database Systems".to_string(),
                "Computer Networks".to_string(),
            ]),
        }
    }
}
