use regex::Regex;
use std::{
    collections::HashMap,
    fs::{read_to_string, write},
    path::{Path, PathBuf},
    sync::Mutex,
    sync::OnceLock,
};

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

pub fn generate_substituted_html(src: &Path, dest: &Path, components_dir: &Path) {
    let mut contents = read_to_string(src).unwrap();
    contents = substitute_replace(&contents, components_dir);
    write(dest, contents).unwrap();
}

fn substitute_replace(contents: &str, components_dir: &Path) -> String{
    let re = Regex::new(r#"<REPLACE\b[^>]*\bwith="([^"]*)"[^>]*/?>"#).unwrap();

    re.replace_all(contents, |caps: &regex::Captures|{
        let with = caps.get(1).unwrap().as_str();
        let path = components_dir.join(with);
        match get_component(&path){
            Ok(s) => s,
            Err(_) => format!("<!-- missing component: {} -->", with) // TODO: Throw error 
        }
    }).into_owned()
}
