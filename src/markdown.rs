use chrono::prelude::*;
use std::{fs::File, io::Write, path::Path, process::Command};
use pathdiff::diff_paths;

// user input name -> path to dir -> markdown file
pub fn create_post(post_name: &str, output_dir_path: &Path) -> () {
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

    let mut file = File::create(md_path).unwrap();
    writeln!(&mut file, "---").unwrap();
    writeln!(&mut file, "title: {post_name}").unwrap();
    writeln!(&mut file, "date: {md_date}").unwrap();
    writeln!(&mut file, "---").unwrap();
}

pub fn render_post(
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
