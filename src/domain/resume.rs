use crate::domain::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Resume {
    basics: Basics,
    work: Option<Vec<Work>>,
    education: Option<Vec<Education>>,
    certificates: Option<Vec<Certificate>>,
    references: Option<Vec<Reference>>,
    skills: Option<Vec<Skill>>,
    awards: Option<Vec<Award>>,
    publications: Option<Vec<Publication>>,
    volunteer: Option<Vec<Volunteer>>,
    projects: Option<Vec<Project>>,
    languages: Option<Vec<Language>>,
    interests: Option<Vec<Interest>>,
}

impl Default for Resume {
    fn default() -> Self {
        Self {
            basics: Basics::default(),
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
    name: String,
    label: String,
    email: String,
    phone: String,
    image: Option<String>,
    summary: Option<String>,
    website: Option<String>,
    url: Option<String>,
    location: Option<Location>,
    profiles: Option<Vec<Profile>>,
}

impl Default for Basics {
    fn default() -> Self {
        Self {
            name: "John Doe".to_string(),
            label: "Software Developer".to_string(),
            email: "john.doe@example.com".to_string(),
            phone: "+1 (123) 456-7890".to_string(),
            image: Some("https://www.example.com/johndoe.jpg".to_string()),
            summary: Some("Passionate software developer with experience in Rust. I enjoy building awesome things!".to_string()),
            website: Some("https://johndoe.com".to_string()),
            url: None,
            location: Some(Location::default()),
            profiles: Some(vec![Profile::default()]),
        }
    }
}
