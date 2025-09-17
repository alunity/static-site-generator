use std::{
    error::Error, fs::{create_dir, read_dir, write}, path::Path
};

use crate::markdown::create_post;

pub fn create_project(dir: impl AsRef<Path>) -> Result<(), Box<dyn Error>> {
    let dir = dir.as_ref();

    if read_dir(dir)?.next().is_some() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            format!("project directory '{}' is not empty", dir.display()),
        ).into());
    }

    let src_path = dir.join("src");
    create_dir(&src_path)?;

    write(src_path.join("index.html"), DEFAULT_INDEX)?;
    write(src_path.join("styles.css"), DEFAULT_CSS)?;
    write(src_path.join("feed.html"), DEFAULT_FEED)?;

    let components_path = src_path.join("components");
    create_dir(&components_path)?;
    write(components_path.join("header.html"), DEFAULT_HEADER)?;
    write(components_path.join("footer.html"), DEFAULT_FOOTER)?;
    write(components_path.join("feed_post.html"), DEFAULT_FEED_POST)?;

    let posts_path = src_path.join("posts");
    create_dir(&posts_path)?;
    create_post("Example Post", &posts_path);
    create_dir(posts_path.join("attachments"))?;

    Ok(())

    // let component_path...
}

const DEFAULT_INDEX: &str = r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <title>My Site</title>
    <link rel="stylesheet" href="./styles.css" />
  </head>
  <body>
    <REPLACE with="header" />

    <h1>Hello</h1>

    <REPLACE with="footer" />
  </body>
</html>
"#;

const DEFAULT_CSS: &str = "/* Add your styles */\n";

const DEFAULT_FEED: &str = "<!-- feed placeholder -->\n";

const DEFAULT_HEADER: &str = "header\n";
const DEFAULT_FOOTER: &str = "footer\n";
const DEFAULT_FEED_POST: &str = "wip\n";