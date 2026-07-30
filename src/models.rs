use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Anime {
    pub title_english: Option<String>,
    pub title_romaji: Option<String>,
    pub members: u32,
    pub score: Option<f64>,
    pub scored_by: Option<f64>,
    pub favorites: u32,   
    pub studios: Option<String>,
    pub format: Option<String>,
    pub episodes: Option<f64>,
    pub genres: Option<String>,
    pub tags: Option<String>,
    pub season: Option<String>,
}

impl Anime {
    pub fn display_title(&self) -> &str {
        if let Some(title) = &self.title_english {
            if !title.trim().is_empty() { return title; }
        }
        if let Some(title) = &self.title_romaji {
            if !title.trim().is_empty() { return title; }
        }
        "Unknown Title"
    }
}
