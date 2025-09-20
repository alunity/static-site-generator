// Implement REPLACE tag on html
mod config;
mod default;
mod html;
mod markdown;

use std::{
    fs::{DirEntry, create_dir, create_dir_all, read_dir, remove_dir_all},
    path::{Path, PathBuf},
};

use pathdiff::diff_paths;

use crate::{
    config::{Config, read_config},
    html::generate_substituted_html,
};

fn main() {
    build(&PathBuf::from("site"));
}

fn build(site_dir: &Path) {
    let c = read_config(&site_dir.join("config.json"));
    let posts_dir = site_dir.join(c.posts_dir);
    let components_dir = site_dir.join(c.components_dir);
    let _ = site_dir.join(c.styles_css);

    let build_dir = site_dir.join("static");
    let src_dir = site_dir.join("src");
    let _ = remove_dir_all(&build_dir);
    create_dir(&build_dir).unwrap();
    // let to_process: Vec<DirEntry> = std::fs::read_dir(src_dir)
    //     .unwrap()
    //     .filter_map(|r| match r {
    //         Ok(e) => Some(e),
    //         Err(e) => {
    //             eprintln!("entry error: {}", e);
    //             None
    //         }
    //     })
    //     .collect();
    //

    let blacklist = vec![components_dir.clone(), posts_dir.clone()];

    let mut stack = vec![src_dir.clone()];
    while let Some(path) = stack.pop() {
        for entry in read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let p = entry.path();

            // TODO: Add some blacklist instead of hard coding
            if blacklist.iter().all(|x| x != &p) {
                if p.is_dir() {
                    stack.push(p);
                } else {
                    let extension = p.extension().unwrap();
                    if extension == "html" {
                        let dest = build_dir.join(diff_paths(&p, &src_dir).unwrap());

                        let _ = create_dir_all(&dest.parent().unwrap());

                        generate_substituted_html(&p, &dest, &components_dir);
                    }
                }
            }
        }
    }
}
