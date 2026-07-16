use crate::domain::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Resume {
    #[serde(skip_serializing_if = "Option::is_none")]
    basics: Option<Basics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    work: Option<Vec<Work>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    education: Option<Vec<Education>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificates: Option<Vec<Certificate>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    references: Option<Vec<Reference>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skills: Option<Vec<Skill>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    awards: Option<Vec<Award>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    publications: Option<Vec<Publication>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volunteer: Option<Vec<Volunteer>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    projects: Option<Vec<Project>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    languages: Option<Vec<Language>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interests: Option<Vec<Interest>>,
}

impl Default for Resume {
    fn default() -> Self {
        Self {
            basics: Some(Basics::default()),
            work: Some(vec![Work::default()]),
            education: Some(vec![Education::default()]),
            certificates: Some(vec![Certificate::default()]),
            references: Some(vec![Reference::default()]),
            skills: Some(vec![Skill::default()]),
            awards: None,
            publications: None,
            volunteer: None,
            projects: Some(vec![Project::default()]),
            languages: Some(vec![Language::default()]),
            interests: Some(vec![Interest::default()]),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Basics {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    website: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<Location>,
    #[serde(skip_serializing_if = "Option::is_none")]
    profiles: Option<Vec<Profile>>,
}

impl Default for Basics {
    fn default() -> Self {
        Self {
            name: Some("John Doe".to_string()),
            label: Some("Software Developer".to_string()),
            email: Some("john.doe@example.com".to_string()),
            phone: Some("+1 (123) 456-7890".to_string()),
            image: Some("https://www.example.com/johndoe.jpg".to_string()),
            summary: Some("Passionate software developer with experience in Rust. I enjoy building awesome things!".to_string()),
            website: Some("https://johndoe.com".to_string()),
            url: None,
            location: Some(Location::default()),
            profiles: Some(vec![Profile::default()]),
        }
    }
}
