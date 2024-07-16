use chrono::{NaiveDate};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct WebsiteConfig {
    pub website: GlobalWebsiteConfig,
    pub render: WebsiteRenderOptions
}

#[derive(Deserialize, Debug)]
pub struct GlobalWebsiteConfig {
    pub name: String,
    pub href: String,
    pub author: Author,
    pub nav: Vec<NavEntry>
}

#[derive(Deserialize, Debug)]
pub struct WebsiteRenderOptions {
    pub include_draft: bool
}

#[derive(Deserialize, Debug)]
pub struct Author {
    pub given_name: String,
    pub family_name: String,
    pub id: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NavEntry {
    pub name: String,
    pub slug: String
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PostMetadata {
    pub title: String,
    pub slug: String,
    pub internal: Option<bool>,
    pub assets: Option<Vec<String>>,
    // Internal date field, use parse_date to get more info
    date: Option<String>,
    draft: Option<bool>,
}
impl PostMetadata {
    pub fn parse_date(&self) -> Option<NaiveDate> {
        if let Some(date_str) = &self.date {
            let parse_result = NaiveDate::parse_from_str(date_str, "%Y-%m-%d");
            
            if let Ok(parsed_date) = parse_result {
                return Some(parsed_date)
            }
        }

        None
    }

    pub fn is_draft(&self) -> bool {
        if let Some(draft) = self.draft {
            return draft;
        }
        false
    }
}