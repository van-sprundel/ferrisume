use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    #[serde(skip_serializing_if = "Option::is_none")]
    highlights: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    website: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    start_date: Option<String>,
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    end_date: Option<String>,
}

impl Default for Project {
    fn default() -> Self {
        Self {
            name: Some("Personal Portfolio Website".to_string()),
            description: Some(
                "A responsive portfolio website built with Rust and WebAssembly".to_string(),
            ),
            website: Some("https://portfolio.example.com".to_string()),
            start_date: Some("2022-01-10".to_string()),
            end_date: Some("2022-03-25".to_string()),
            highlights: Some(vec![
                "Implemented responsive design".to_string(),
                "Used Rust compiled to WebAssembly for interactive elements".to_string(),
                "Achieved 98/100 performance score on Lighthouse".to_string(),
            ]),
        }
    }
}
