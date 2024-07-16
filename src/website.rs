use chrono::Datelike;
use tera::Context;

use crate::config::WebsiteConfig;

#[derive(Debug)]
pub struct Website {
  pub config: WebsiteConfig
}

impl Website {

  /// Generates a render context with global keys set
  pub fn render_context(&self) -> Context {
    let mut context = Context::new();
    context.insert("current_year", &chrono::Utc::now().year());
    context.insert("website_name", &self.config.website.name);
    context.insert("website_base_href", &self.config.website.href);
    context.insert("author_given_name", &self.config.website.author.given_name);
    context.insert("author_family_name", &self.config.website.author.family_name);
    context.insert("nav_entries", &self.config.website.nav);
    
    context
  }
}