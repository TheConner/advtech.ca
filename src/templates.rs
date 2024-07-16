use tera::Tera;
use lazy_static::lazy_static;

lazy_static! {
  pub static ref TEMPLATES: Tera = {
      let mut tera = match Tera::new("./templates/**/*") {
          Ok(t) => t,
          Err(e) => {
              println!("Parsing error(s): {}", e);
              ::std::process::exit(1);
          }
      };
      tera
  };
}