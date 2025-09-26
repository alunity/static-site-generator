use pathdiff::diff_paths;
use regex::Regex;
use std::{
    collections::HashMap,
    fs::{read_to_string, write},
    path::{Path, PathBuf},
    sync::Mutex,
    sync::OnceLock,
};

use crate::markdown::{MdInfo, get_mdinfos_for_path};

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

pub fn generate_substituted_html(src: &Path, dest: &Path, components_dir: &Path, posts_dir: &Path) {
    let mut contents = read_to_string(src).unwrap();
    contents = substitute_replace(&contents, components_dir);
    contents = substitute_feed(&contents, src, components_dir, posts_dir);
    write(dest, contents).unwrap();
}

fn substitute_replace(contents: &str, components_dir: &Path) -> String {
    let re = Regex::new(r#"<REPLACE\b[^>]*\bwith="([^"]*)"[^>]*/?>"#).unwrap();

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
    let re = Regex::new(r#"<FEED\b[^>]*\bwith="([^"]*)"[^>]*/?>"#).unwrap();

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

                MdInfo {
                    title: c.title.to_owned(),
                    date: c.date,
                    content: c.content.to_owned(),
                    path: diff_paths(new_path, curr_path.parent().unwrap()).unwrap(),
                }
            })
            .map(|c| hydrate_post_component(&component, c))
            .collect();

        hydrated_components.join("\n")
    })
    .into_owned()
}

fn hydrate_post_component(component: &String, post: MdInfo) -> String {
    // Supporting tags: {TITLE}, {DATE}, {CONTENT}, {PATH}
    let mut res = component.replace("{TITLE}", &post.title);
    res = res.replace("{DATE}", &post.date.format("%A %d %B %Y").to_string());
    res = res.replace("{CONTENT}", &post.content);
    res = res.replace("{PATH}", &post.path.to_string_lossy());

    res
}
