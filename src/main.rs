use std::fs;
use std::io;
use std::path::Path;

use config::PostMetadata;
use content::render_styles;
use content::render_tags;

mod config;
mod content;
mod templates;
mod util;
mod website;

use crate::config::WebsiteConfig;
use crate::content::clear_output;
use crate::content::render_index;
use crate::content::render_content;
use crate::website::Website;

fn main() {
    let config: WebsiteConfig = toml::from_str(
        fs::read_to_string("Website.toml")
            .expect("Could not load config")
            .as_str(),
    )
    .unwrap();

    println!("Loaded website config");

    let website = Website { config: config };

    render(website).expect("Could not render website");

    return;
}

fn render(website: Website) -> io::Result<()> {
    clear_output().expect("Could not clear output");

    render_styles()?;  

    let all_content_metadata = render_content(&Path::new("./content"), &website);
    
    let mut all_post_metadata: Vec<PostMetadata> = all_content_metadata.iter()
        .cloned()
        .filter(|post| {
            if let Some(internal) = post.internal {
                !internal
            } else {
                true
            }
        })
        .collect();

    // All posts rendered, render tag summary pages
    render_tags(&website, all_content_metadata).expect("Could not render tags");
    
    all_post_metadata.sort_by_key(|post| {
        post.parse_date()
    });

    all_post_metadata.reverse();

    render_index(&website, all_post_metadata).expect("Fuck");

    Ok(())
}
