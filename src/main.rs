// Implement REPLACE tag on html
mod config;
mod default;
mod html;
mod markdown;

use std::path::{Path, PathBuf};

fn main() {
    println!(
        "{:?}",
        config::read_config(&PathBuf::from("site/config.json"))
    ) // default::create_project(PathBuf::from("site")).unwrap();
    // html::generate_substituted_html(Path::new("site/src/index.html"), Path::new("test.html"), Path::new("site/src/components"));
}
