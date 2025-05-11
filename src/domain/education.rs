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

impl Default for Education {
    fn default() -> Self {
        Self {
            start_date: "2014-09-01".to_string(),
            end_date: Some("2018-05-15".to_string()),
            area: "Computer Science".to_string(),
            study_type: "Bachelor of Science".to_string(),
            institution: "University of Technology".to_string(),
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
