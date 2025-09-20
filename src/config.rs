use std::{
    default,
    fs::read_to_string,
    io::Result,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub styles_css: PathBuf,
    pub components_dir: PathBuf,
    pub posts_dir: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            styles_css: PathBuf::from("src/styles.css"),
            components_dir: PathBuf::from("src/components"),
            posts_dir: PathBuf::from("src/posts"),
        }
    }
}

pub fn read_config(path: &Path) -> Config {
    let res: Config = serde_json::from_str(&read_to_string(path).unwrap()).unwrap();
    res
}
