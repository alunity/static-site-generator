use std::{
    error::Error,
    fs::{create_dir, read_dir, write},
    path::Path,
};

use crate::{config::Config, markdown::create_post};

pub fn create_project(dir: impl AsRef<Path>) -> Result<(), Box<dyn Error>> {
    let dir = dir.as_ref();

    if read_dir(dir)?.next().is_some() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            format!("project directory '{}' is not empty", dir.display()),
        )
        .into());
    }

    // write!(dir.join("config.json"), Config::default());
    //
    write(
        dir.join("config.json"),
        serde_json::to_string(&Config::default()).unwrap(),
    )?;

    let src_path = dir.join("src");
    create_dir(&src_path)?;

    write(src_path.join("index.html"), DEFAULT_INDEX)?;
    write(src_path.join("styles.css"), DEFAULT_CSS)?;
    write(src_path.join("feed.html"), DEFAULT_FEED)?;

    let components_path = src_path.join("components");
    create_dir(&components_path)?;
    write(components_path.join("header.html"), DEFAULT_HEADER)?;
    write(components_path.join("footer.html"), DEFAULT_FOOTER)?;
    write(components_path.join("post.html"), DEFAULT_FEED_POST)?;

    let templates_path = src_path.join("templates");
    create_dir(templates_path)?;
    write(components_path.join("post.html"), DEFAULT_POST)?;

    let posts_path = src_path.join("posts");
    create_dir(&posts_path)?;
    create_post("Example Post", &posts_path);
    create_dir(posts_path.join("attachments"))?;

    Ok(())
}

const DEFAULT_INDEX: &str = r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <title>My Site</title>
    <link rel="stylesheet" href="./styles.css" />
  </head>
  <body>
    <REPLACE with="header.html" />

    <h1>Hello</h1>

    <REPLACE with="footer.html" />
  </body>
</html>
"#;

const DEFAULT_CSS: &str = "/* Add your styles */\n";

const DEFAULT_FEED: &str = "<FEED with=\"post.html\">\n";

const DEFAULT_HEADER: &str = "<div>header</div>\n";
const DEFAULT_FOOTER: &str = "<div>footer</div>\n";
const DEFAULT_FEED_POST: &str = "wip\n";
const DEFAULT_POST: &str = r#"<!DOCTYPE html>
<html xmlns="http://www.w3.org/1999/xhtml" lang="" xml:lang="">
<head>
  <meta charset="utf-8" />
  <meta name="generator" content="pandoc" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0, user-scalable=yes" />
  <title>{TITLE}</title>
  <style>
    code{white-space: pre-wrap;}
    span.smallcaps{font-variant: small-caps;}
    div.columns{display: flex; gap: min(4vw, 1.5em);}
    div.column{flex: auto; overflow-x: auto;}
    div.hanging-indent{margin-left: 1.5em; text-indent: -1.5em;}
    /* The extra [class] is a hack that increases specificity enough to
       override a similar rule in reveal.js */
    ul.task-list[class]{list-style: none;}
    ul.task-list li input[type="checkbox"] {
      font-size: inherit;
      width: 0.8em;
      margin: 0 0.8em 0.2em -1.6em;
      vertical-align: middle;
    }
  </style>
  <link rel="stylesheet" href="{CSSPATH}" />
</head>
<body>
<header id="title-block-header">
<h1 class="title">{TITLE}</h1>
<p class="date">{DATE}</p>
</header>
{CONTENT}
</body>
</html>
"#;
