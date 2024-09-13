use crate::domain::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
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

#[derive(Debug, Default, Serialize, Deserialize)]
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
