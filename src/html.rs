use pathdiff::diff_paths;
use regex::Regex;
use std::{
    collections::HashMap,
    fs::{read_to_string, write},
    path::{Path, PathBuf},
    sync::Mutex,
    sync::OnceLock,
};

use crate::{config::Config, markdown::{get_mdinfos_for_path, truncate_content}, rss::add_rss_meta};

// Simple per-process cache for component files
static COMPONENT_CACHE: OnceLock<Mutex<HashMap<PathBuf, String>>> = OnceLock::new();

fn get_component(path: &Path) -> std::io::Result<String> {
    let cache = COMPONENT_CACHE.get_or_init(|| Mutex::new(HashMap::new()));
    let mut map = cache.lock().unwrap();

    if let Some(s) = map.get(path) {
        return Ok(s.clone());
    }
    let s = read_to_string(path)?;
    map.insert(path.to_path_buf(), s.clone());
    Ok(s)
}

pub fn generate_substituted_html(src: &Path, dest: &Path, posts_dir: &Path, components_dir: &Path ,config: &Config) {
    let mut contents = read_to_string(src).unwrap();
    contents = substitute_replace(&contents, components_dir);
    contents = substitute_feed(&contents, src, components_dir, posts_dir);
    contents = add_rss_meta(&contents, &config.hosted_url, &config.site_name);
    write(dest, contents).unwrap();
}

fn substitute_replace(contents: &str, components_dir: &Path) -> String {
    let re = Regex::new(r#"<REPLACE\b[^>]*\bwith="([^"]*)"[^>]*/>"#).unwrap();

    re.replace_all(contents, |caps: &regex::Captures| {
        let with = caps.get(1).unwrap().as_str();
        let path = components_dir.join(with);
        match get_component(&path) {
            Ok(s) => s,
            Err(_) => format!("<!-- missing component: {} -->", with), // TODO: Throw error
        }
    })
    .into_owned()
}

fn substitute_feed(
    contents: &str,
    curr_path: &Path,
    components_dir: &Path,
    posts_dir: &Path,
) -> String {
    let re = Regex::new(r#"<FEED\b[^>]*\bwith="([^"]*)"[^>]*/>"#).unwrap();

    re.replace_all(contents, |caps: &regex::Captures| {
        let with = caps.get(1).unwrap().as_str();
        let path = components_dir.join(with);

        let component = get_component(&path).unwrap();

        let mut mdinfos = get_mdinfos_for_path(&posts_dir).unwrap();
        mdinfos.sort();
        mdinfos.reverse();
        let hydrated_components: Vec<String> = mdinfos
            .iter()
            .map(|c| {
                let mut new_path = c.path.clone();
                new_path.set_extension("html");

                HashMap::from([
                    ("TITLE", c.title.to_owned()),
                    ("DATE", c.date.format("%A %d %B %Y").to_string()),
                    ("CONTENT", truncate_content(&c.content, 160)),
                    ("PATH", diff_paths(new_path, curr_path.parent().unwrap()).unwrap().to_string_lossy().to_string())
                ])
            })
            .map(|c| hydrate_component(&component, c))
            .collect();

        hydrated_components.join("\n")
    })
    .into_owned()
}

fn hydrate_component(component: &str, fields: HashMap<&str, String>) -> String {
    let re = Regex::new(r"\{([[:alpha:]]*)\}").unwrap();

    re.replace_all(&component, |caps: &regex::Captures| {
        let text = caps.get(0).unwrap().as_str();
        let field = caps.get(1).unwrap().as_str();
        if fields.contains_key(field) {
            fields.get(field).unwrap().to_string()
        } else {
            text.to_string()
        }
    }).to_string()
}


