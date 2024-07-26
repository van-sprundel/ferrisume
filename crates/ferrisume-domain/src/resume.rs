use crate::award::Award;
use crate::interest::Interest;
use crate::language::Language;
use crate::location::Location;
use crate::profile::Profile;
use crate::project::Project;
use crate::publication::Publication;
use crate::reference::Reference;
use crate::skill::Skill;
use crate::volunteer::Volunteer;
use crate::work::Work;
use crate::{certificate::Certificate, education::Education};
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
