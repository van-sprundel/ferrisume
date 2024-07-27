use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Publication {
    name: String,
    publisher: String,
    #[serde(rename = "releaseDate")]
    release_date: String,
    url: Option<String>,
    summary: Option<String>,
}
