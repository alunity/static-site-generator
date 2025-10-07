use pathdiff::diff_paths;
use regex::Regex;
use std::{
    collections::HashMap,
    fmt::Debug,
    fs::{read_to_string, write},
    path::{Path, PathBuf},
    sync::{Mutex, OnceLock},
};
use thiserror::Error;

use crate::{
    config::Config,
    markdown::{MdError, get_mdinfos_for_path, truncate_content},
    rss::add_rss_meta,
};

#[derive(Debug, Error)]
pub enum HTMLError {
    #[error("Error reading component {path}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("MdError {md_error}")]
    MdInfo { md_error: MdError },
    #[error("Missing field {tag}")]
    MissingField { tag: String },
}

pub type Result<T> = std::result::Result<T, HTMLError>;

// Simple per-process cache for component files
static COMPONENT_CACHE: OnceLock<Mutex<HashMap<PathBuf, String>>> = OnceLock::new();

fn get_component(path: &Path) -> Result<String> {
    let cache = COMPONENT_CACHE.get_or_init(|| Mutex::new(HashMap::new()));
    let mut map = cache.lock().unwrap();

    if let Some(s) = map.get(path) {
        return Ok(s.clone());
    }
    let s = read_to_string(path).map_err(|e| HTMLError::Io {
        path: PathBuf::from(path),
        source: e,
    })?;
    map.insert(path.to_path_buf(), s.clone());
    Ok(s)
}

pub fn generate_substituted_html(
    src: &Path,
    dest: &Path,
    posts_dir: &Path,
    components_dir: &Path,
    config: &Config,
) -> Result<()> {
    let mut contents = read_to_string(src).unwrap();
    contents = substitute_replace(&contents, components_dir)?;
    contents = substitute_feed(&contents, src, components_dir, posts_dir)?;
    contents = add_rss_meta(&contents, &config.hosted_url, &config.site_name);
    write(dest, contents).unwrap();
    Ok(())
}

pub fn substitute_replace(contents: &str, components_dir: &Path) -> Result<String> {
    let re = Regex::new(r#"<REPLACE\b[^>]*\bwith="([^"]*)"[^>]*/>"#).unwrap();

    let mut out = String::with_capacity(contents.len());
    let mut last_end = 0;

    for caps in re.captures_iter(contents) {
        let whole = caps.get(0).unwrap();
        out.push_str(&contents[last_end..whole.start()]);

        let with = caps
            .get(1)
            .ok_or_else(|| HTMLError::MissingField { tag: "with".into() })?
            .as_str();

        let path = components_dir.join(with);
        let component = get_component(&path)?;

        out.push_str(&component);
        last_end = whole.end();
    }

    out.push_str(&contents[last_end..]);
    Ok(out)
}

fn substitute_feed(
    contents: &str,
    curr_path: &Path,
    components_dir: &Path,
    posts_dir: &Path,
) -> Result<String> {
    // Precompute data that is the same for every match
    let mut mdinfos =
        get_mdinfos_for_path(posts_dir).map_err(|e| HTMLError::MdInfo { md_error: e })?; // was unwrap
    mdinfos.sort();
    mdinfos.reverse();

    // We will walk matches and splice replacements
    let re = Regex::new(r#"<FEED\b[^>]*\bwith="([^"]*)"[^>]*/>"#).expect("Regex fail how"); // hard-coded? make it static and expect() instead
    let mut out = String::with_capacity(contents.len());
    let mut last_end = 0;

    for caps in re.captures_iter(contents) {
        let m = caps.get(0).unwrap(); // whole match span
        out.push_str(&contents[last_end..m.start()]);

        let with = caps
            .get(1)
            .ok_or_else(|| HTMLError::MissingField { tag: "with".into() })?;
        // .ok_or_else(|| anyhow::anyhow!("expected capture group 1"))?
        // .as_str();
        //
        let component_path = components_dir.join(with.as_str());
        let component_tpl = get_component(&component_path)?; // now ? works

        // Build hydrated components once per match (if it really varies by `with`)
        let hydrated = mdinfos
            .iter()
            .map(|c| {
                let mut new_path = c.path.clone();
                new_path.set_extension("html");
                let rel = diff_paths(&new_path, curr_path.parent().unwrap())
                    .unwrap()
                    .to_string_lossy()
                    .to_string();
                let map = HashMap::from([
                    ("TITLE", c.title.clone()),
                    ("DATE", c.date.format("%A %d %B %Y").to_string()),
                    ("CONTENT", truncate_content(&c.content, 160)),
                    ("PATH", rel),
                ]);
                hydrate_component(&component_tpl, map)
            })
            .collect::<Vec<String>>(); // if hydrate_component returns Result<String>

        out.push_str(&hydrated.join("\n"));

        last_end = m.end();
    }

    out.push_str(&contents[last_end..]);
    Ok(out)
}

fn hydrate_component(component: &str, fields: HashMap<&str, String>) -> String {
    let re = Regex::new(r"\{([[:alpha:]]*)\}").unwrap();

    re.replace_all(&component, |caps: &regex::Captures| {
        let key = &caps[1]; // capture group 1
        fields.get(key).map(|s| s.as_str()).unwrap_or(&caps[0]).to_owned() // original "{KEY}"
    })
    .to_string()
}
