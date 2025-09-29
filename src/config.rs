use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub styles_css: PathBuf,
    pub components_dir: PathBuf,
    pub posts_dir: PathBuf,
    pub hosted_url: String,
    pub og_image_url: String,
    pub site_name: String,
    pub description: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            styles_css: PathBuf::from("src/styles.css"),
            components_dir: PathBuf::from("src/components"),
            posts_dir: PathBuf::from("src/posts"),
            hosted_url: "https://example.com".to_owned(),
            og_image_url: "https://upload.wikimedia.org/wikipedia/en/a/a9/Example.jpg".to_owned(),
            site_name: "My Site".to_owned(),
            description: "My lovely website".to_owned(),
        }
    }
}

pub fn read_config(path: &Path) -> Config {
    let res: Config = serde_json::from_str(&read_to_string(path).unwrap()).unwrap();
    res
}
