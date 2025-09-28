// Implement REPLACE tag on html
mod config;
mod default;
mod html;
mod markdown;

use std::{
    fs::{copy, create_dir, create_dir_all, read_dir, remove_dir_all, write},
    path::{Path, PathBuf},
    process::Command,
};

use clap::{Parser, Subcommand};
use pathdiff::diff_paths;

use crate::{
    config::read_config,
    html::generate_substituted_html,
    markdown::{add_meta_to_post_html, get_mdinfos_for_path, render_to_html},
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    // Project Path
    path: PathBuf,
}

#[derive(Subcommand)]
enum Commands {
    /// Builds static site
    Build {
        /// Output dir
        #[arg(short, long)]
        output_dir: Option<PathBuf>,
    },
    /// Creates new site
    Init,
    /// Creates new post
    ///
    Post {
        name: String,
        open_in_editor: Option<bool>,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Build { output_dir } => {
            if let Some(path) = output_dir {
                build(&cli.path, &path);
            } else {
                build(&cli.path, &cli.path.join("static"));
            }
        }
        Commands::Init => default::create_project(&cli.path).unwrap(),
        Commands::Post {
            name,
            open_in_editor,
        } => {
            let md_path = markdown::create_post(
                name,
                &cli.path
                    .join(&read_config(&cli.path.join("config.json")).posts_dir),
            );
            if let Some(open) = open_in_editor
                && *open
            {
                if let Ok(editor) = std::env::var("EDITOR") {
                    Command::new(editor).arg(&md_path).status().ok();
                } else {
                    println!("$EDITOR not set; cannot open file.");
                }
            }
        }
    }
}

fn build(site_dir: &Path, build_dir: &Path) {
    let c = read_config(&site_dir.join("config.json"));
    let posts_dir = site_dir.join(c.posts_dir);
    let components_dir = site_dir.join(c.components_dir);
    let styles_css = site_dir.join(c.styles_css);
    let url = c.hosted_url;
    let og_image_url = c.og_image_url;

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
                        Some("html") => {
                            generate_substituted_html(&p, &dest, &components_dir, &posts_dir)
                        }
                        Some("md") => {
                            let styles_css =
                                build_dir.join(diff_paths(&styles_css, &src_dir).unwrap());
                            dest.set_extension("html");
                            let html = render_to_html(&p, &dest, Some(&styles_css), None, None);
                            let q = get_mdinfos_for_path(p.parent().unwrap()).unwrap();
                            let c = q.iter().filter(|c| c.path == p).next().unwrap();
                            let post_url = url.clone() + "/"
                                + &diff_paths(&dest, &build_dir)
                                    .unwrap()
                                    .to_string_lossy()
                                    .to_string();

                            write(dest, add_meta_to_post_html(html, c, &post_url, &og_image_url)).unwrap();
                        }
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
