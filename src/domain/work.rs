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

impl Default for Work {
    fn default() -> Self {
        Self {
            name: "Acme Corporation".to_string(),
            position: "Senior Developer".to_string(),
            url: "https://www.acmecorp.com".to_string(),
            start_date: "2018-01-15".to_string(),
            end_date: Some("2021-06-30".to_string()),
            summary: "Led development of the company's flagship product.".to_string(),
            highlights: Some(vec![
                "Increased performance by 40%".to_string(),
                "Implemented CI/CD pipeline".to_string(),
                "Mentored junior developers".to_string(),
            ]),
        }
    }
}
