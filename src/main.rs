mod config;
mod default;
mod html;
mod markdown;
mod rss;

use std::{
    fs::{copy, create_dir, create_dir_all, read_dir, remove_dir_all, write},
    path::{Path, PathBuf},
    process::Command,
};

use chrono::{DateTime, Utc};
use clap::{Parser, Subcommand};
use pathdiff::diff_paths;

use rss_gen::{RssData, RssItem, RssVersion, generate_rss};

use crate::{
    config::read_config,
    html::generate_substituted_html,
    markdown::{add_meta_to_post_html, get_mdinfos_for_path, render_to_html, truncate_content},
    rss::add_rss_meta,
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
            )
            .unwrap();
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
    let posts_dir = site_dir.join(&c.posts_dir);
    let components_dir = site_dir.join(&c.components_dir);
    let styles_css = site_dir.join(&c.styles_css);

    let src_dir = site_dir.join("src");
    let _ = remove_dir_all(&build_dir);
    create_dir(&build_dir).unwrap();
    let blacklist = vec![components_dir.clone()];

    let mut rss_data = RssData::new(Some(RssVersion::RSS2_0))
        .title(&c.site_name)
        .link(&c.hosted_url)
        .description(&c.description);

    let mut stack = vec![src_dir.clone()];
    while let Some(path) = stack.pop() {
        for entry in read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let p = entry.path();

            if blacklist.iter().all(|x| x != &p) {
                if p.is_dir() {
                    stack.push(p);
                } else {
                    let mut dest = build_dir.join(diff_paths(&p, &src_dir).unwrap());
                    let _ = create_dir_all(&dest.parent().unwrap());

                    match p.extension().and_then(|s| s.to_str()) {
                        Some("html") => {
                            generate_substituted_html(&p, &dest, &posts_dir, &components_dir, &c).unwrap();
                        }
                        Some("md") => {
                            let styles_css =
                                build_dir.join(diff_paths(&styles_css, &src_dir).unwrap());
                            dest.set_extension("html");
                            let html = render_to_html(&p, &dest, Some(&styles_css), None, None);

                            let md_infos = get_mdinfos_for_path(p.parent().unwrap()).unwrap();
                            let md_info = md_infos.iter().filter(|c| c.path == p).next().unwrap();

                            let post_url = c.hosted_url.clone()
                                + "/"
                                + &diff_paths(&dest, &build_dir)
                                    .unwrap()
                                    .to_string_lossy()
                                    .to_string();

                            write(
                                dest,
                                add_rss_meta(
                                    &add_meta_to_post_html(
                                        html,
                                        md_info,
                                        &post_url,
                                        &c.og_image_url,
                                        &c.site_name,
                                    ),
                                    &c.hosted_url,
                                    &c.site_name,
                                ),
                            )
                            .unwrap();

                            rss_data.add_item(
                                RssItem::new()
                                    .title(&md_info.title)
                                    .description(truncate_content(&md_info.content, 80))
                                    .guid(&post_url)
                                    .pub_date(
                                        DateTime::<Utc>::from_naive_utc_and_offset(
                                            md_info.date.and_hms_opt(0, 0, 0).unwrap(),
                                            Utc,
                                        )
                                        .to_rfc2822(),
                                    )
                                    .link(&post_url),
                            );
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
    // gen rss
    //
    write(build_dir.join("feed.xml"), generate_rss(&rss_data).unwrap()).unwrap();
}
