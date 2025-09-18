// Implement REPLACE tag on html
mod default;
mod html;
mod markdown;

use std::path::Path;

fn main() {
    // let p = Path::new("test/src/posts/25_09_16_unified_theory_of_programming_.md");
    // let q = Path::new("test/src/posts/q.html");
    // let css = Path::new("test/src/styles.css");
    // let snippet = Path::new("test/snippet.html");

    // markdown::render_post(p, q, Some(css), Some(snippet), None);
    // default::create_project(Path::new("site")).unwrap();
    html::generate_substituted_html(Path::new("site/src/index.html"), Path::new("test.html"), Path::new("site/src/components"));
}
