// Implement REPLACE tag on html
mod config;
mod default;
mod html;
mod markdown;

use std::{
    fs::{DirEntry, copy, create_dir, create_dir_all, read_dir, remove_dir_all},
    path::{Path, PathBuf},
};

use pathdiff::diff_paths;

use crate::{
    config::{Config, read_config},
    html::generate_substituted_html,
    markdown::render_to_html,
};

fn main() {
    build(&PathBuf::from("site"));
}

fn build(site_dir: &Path) {
    let c = read_config(&site_dir.join("config.json"));
    let posts_dir = site_dir.join(c.posts_dir);
    let components_dir = site_dir.join(c.components_dir);
    let styles_css = site_dir.join(c.styles_css);

    let build_dir = site_dir.join("static");
    let src_dir = site_dir.join("src");
    let _ = remove_dir_all(&build_dir);
    create_dir(&build_dir).unwrap();
    let blacklist = vec![components_dir.clone()];

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
                    let mut dest = build_dir.join(diff_paths(&p, &src_dir).unwrap());
                    let _ = create_dir_all(&dest.parent().unwrap());

                    match p.extension().and_then(|s| s.to_str()) {
                        Some("html") => generate_substituted_html(&p, &dest, &components_dir),
                        Some("md") => {
                            let styles_css = build_dir.join(diff_paths(&styles_css, &src_dir).unwrap());
                            dest.set_extension("html");
                            render_to_html(&p, &dest, Some(&styles_css), None, None)
                        },
                        Some(_) => {
                            copy(&p, &dest).unwrap();
                        }
                        None => panic!("{}", p.display()),
                    };
                }
            }
        }
    }
}
