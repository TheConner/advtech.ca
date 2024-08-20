use std::fs;
use std::io;
use std::path::Path;

use config::PostMetadata;
use content::render_atom;
use content::render_styles;
use content::render_tags;
use content::Post;

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
    clear_output().ok();

    render_styles()?;  

    let all_content: Vec<Post> = render_content(&Path::new("./content"), &website).collect();
    
    let mut all_content_meta: Vec<PostMetadata> = all_content.iter()
        .filter(|post| post.metadata.is_some())
        .filter(|post| {
            let metadata = post.metadata.as_ref().unwrap();
            if let Some(internal) = metadata.internal {
                return !internal;
            }
            return true;
        })
        .map(|post| post.metadata.clone().unwrap())
        .collect();

    // All posts rendered, render tag summary pages
    render_tags(&website, all_content_meta.clone().into_iter()).expect("Could not render tags");
    
    all_content_meta.sort_by_key(|post| {
        post.parse_date()
    });

    all_content_meta.reverse();

    render_index(&website, &all_content_meta).expect("Fuck");
    render_atom(&website, &all_content).expect("Ouch");

    Ok(())
}
