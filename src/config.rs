use std::{
    fs::read_to_string,
    io,
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

pub type Result<T> = std::result::Result<T, ConfigError>;

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Cannot find error {source}")]
    CannotFindConfig { source: io::Error },
    #[error("Missing fields {source}")]
    MissingFields { source: serde_json::Error },
}

pub fn read_config(path: &Path) -> Result<Config> {
    let res: Config = serde_json::from_str(
        &read_to_string(path).map_err(|e| ConfigError::CannotFindConfig { source: e })?,
    )
    .map_err(|e| ConfigError::MissingFields { source: e })?;
    Ok(res)
}
