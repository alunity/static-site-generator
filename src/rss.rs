pub fn add_rss_meta(contents: &str, url: &str, title: &str) -> String {
    contents.replace("</head>", &format!(r#"
      <link rel="alternate"
        type="application/rss+xml"
        href="{}"
        title="{}">
    </head>"#, String::from(url) + "/feed.xml", title))
}
