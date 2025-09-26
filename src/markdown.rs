use chrono::prelude::*;
use pathdiff::diff_paths;
use regex::Regex;
use serde::Deserialize;
use std::{
    collections::HashMap,
    fs::{File, read_dir, read_to_string},
    io::Write,
    path::{Path, PathBuf},
    process::Command,
    sync::{Mutex, OnceLock},
};

// Simple per-process cache for component files
static POST_CACHE: OnceLock<Mutex<HashMap<PathBuf, Vec<MdInfo>>>> = OnceLock::new();

pub fn get_mdinfos_for_path(posts_dir: &Path) -> std::io::Result<Vec<MdInfo>> {
    let cache = POST_CACHE.get_or_init(|| Mutex::new(HashMap::new()));
    let mut map = cache.lock().unwrap();

    if let Some(s) = map.get(posts_dir) {
        return Ok(s.to_vec());
    }

    let mut stack= vec![PathBuf::from(posts_dir)];
    let mut res: Vec<MdInfo> = vec![];
    while let Some(path) = stack.pop() {
        for entry in read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let p = entry.path();
            if p.is_dir(){
                stack.push(p);
            }else if p.extension().unwrap() == "md"{
                res.push(get_md_info(&p));
            }
        }
    };
    map.insert(posts_dir.to_path_buf(), res.to_vec());
    Ok(res)
}

// user input name -> path to dir -> markdown file
pub fn create_post(post_name: &str, output_dir_path: &Path) -> PathBuf {
    let mut file_safe_name = post_name.to_string();
    // Remove non alphanumeric characters and change spaces to underscores
    let separator = '_';
    file_safe_name = file_safe_name
        .chars()
        .map(|x| if x == ' ' { separator } else { x })
        .filter(|x| x.is_alphanumeric() || *x == separator)
        .collect();

    let local: DateTime<Local> = Local::now();
    let file_safe_date = local.format("%y_%m_%d");
    let md_date = local.format("%A %d %B %Y");

    let md_path = output_dir_path.join(format!("{file_safe_date}_{file_safe_name}.md"));

    let mut file = File::create(&md_path).unwrap();
    writeln!(&mut file, "---").unwrap();
    writeln!(&mut file, "title: {post_name}").unwrap();
    writeln!(&mut file, "date: {md_date}").unwrap();
    writeln!(&mut file, "---").unwrap();
    md_path
}

pub fn render_to_html(
    md_path: &Path,
    output_path: &Path,
    css_path: Option<&Path>,
    header_path: Option<&Path>,
    footer_path: Option<&Path>,
) -> () {
    let mut c = Command::new("pandoc");
    c.arg(md_path).arg("-s");

    if let Some(css_path) = css_path {
        c.arg("-c");
        c.arg(diff_paths(css_path, output_path.parent().unwrap()).unwrap());
    }
    if let Some(header_path) = header_path {
        c.arg("-B");
        c.arg(header_path);
    }
    if let Some(footer_path) = footer_path {
        c.arg("-A");
        c.arg(footer_path);
    }
    c.arg("-o");
    c.arg(output_path);

    c.spawn().unwrap();
}

#[derive(Debug, Deserialize)]
struct FrontMatter {
    title: String,
    date: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MdInfo {
    pub date: NaiveDate,
    pub title: String,
    pub content: String,
    pub path: PathBuf,
}

pub fn get_md_info(path: &Path) -> MdInfo {
    let contents = read_to_string(path).unwrap();
    let re = Regex::new(r"(?s)\A---\s*\n(.*?)\n---\s*\n?(.*)\z").unwrap();

    let caps = re.captures(&contents).unwrap();
    let fm_str = caps.get(1).unwrap().as_str();
    let content = caps.get(2).unwrap().as_str();

    let fm: FrontMatter = serde_yaml::from_str(fm_str).unwrap();
    MdInfo {
        title: fm.title,
        date: parse_date(&fm.date),
        content: content.to_string(),
        path: path.into(),
    }
}

fn parse_date(date_str: &str) -> NaiveDate {
    // Example date: "Tuesday 16 September 2025"
    // Format: weekday full name, space-padded day, month full name, year
    // Chrono format: "%A %e %B %Y"
    NaiveDate::parse_from_str(date_str, "%A %e %B %Y")
        .or_else(|_| NaiveDate::parse_from_str(date_str, "%e %B %Y"))
        .unwrap() // fallback without weekday
}
