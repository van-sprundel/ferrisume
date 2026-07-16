use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Work {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    position: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    start_date: Option<String>,
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    end_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    highlights: Option<Vec<String>>,
}

impl Default for Work {
    fn default() -> Self {
        Self {
            name: Some("Acme Corporation".to_string()),
            position: Some("Senior Developer".to_string()),
            url: Some("https://www.acmecorp.com".to_string()),
            start_date: Some("2018-01-15".to_string()),
            end_date: Some("2021-06-30".to_string()),
            summary: Some("Led development of the company's flagship product.".to_string()),
            highlights: Some(vec![
                "Increased performance by 40%".to_string(),
                "Implemented CI/CD pipeline".to_string(),
                "Mentored junior developers".to_string(),
            ]),
        }
    }
}
