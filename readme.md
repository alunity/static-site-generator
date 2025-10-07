# static_site_generator

Minimal Rust static site & blog generator with:

- Markdown -> HTML via Pandoc (with MathJax support)
- Component inclusion tags
- Automatic feed page generation
- Open Graph + RSS metadata
- RSS 2.0 feed (`feed.xml`)
- Simple project bootstrap

---

## Quick Start

```bash
cargo install --path .
static_site_generator Init my_site
cd my_site
static_site_generator Post "My First Post" .
# edit generated src/posts/YY_MM_DD_my_first_post.md
static_site_generator Build .
# output in ./static
```

Ensure `pandoc` is installed and on PATH.

---

## CLI

Defined in [src/main.rs](src/main.rs) (`clap`).

Commands (all require a final positional PATH to the project root):

```
static_site_generator build [--output-dir <dir>] <path>
static_site_generator init <path>
static_site_generator post [--open-in-editor <true|false>] <name> <path>
```

- Build: processes `src/` into a mirrored `static/` (or `--output-dir`)
- Init: scaffolds a new site (config, components, example post)
- Post: creates a new Markdown post (opens in $EDITOR if set and not disabled)

Symbols: [`Commands`](src/main.rs), [`markdown::create_post`](src/markdown.rs)

---

## Project Layout

After `init`:

```
config.json
src/
  index.html
  feed.html
  styles.css
  components/
    header.html
    footer.html
    post.html
  posts/
    YY_MM_DD_example_post.md
    attachments/
static/ (generated on Build)
```

---

## config.json

Structure from [`config::Config`](src/config.rs):

```json
{
  "styles_css": "src/styles.css",
  "components_dir": "src/components",
  "posts_dir": "src/posts",
  "hosted_url": "https://example.com",
  "og_image_url": "https://upload.wikimedia.org/wikipedia/en/a/a9/Example.jpg",
  "site_name": "My Site",
  "description": "My lovely website"
}
```

`hosted_url` must be the canonical absolute base (no trailing slash).  
Used for RSS + Open Graph tags.

---

## Tags (Template Directives)

Processed in [`html::generate_substituted_html`](src/html.rs).

### Component Include

```html
<REPLACE with="header.html" />
```

- Loads file from `components_dir`
- Replaces the self-closing tag inline
- Cached in‑process

Resolved by [`html::substitute_replace`](src/html.rs).

### Feed Expansion

```html
<FEED with="post.html" />
```

- Repeated once per post (sorted newest first)
- Template file (e.g. `post.html`) can contain placeholders:
  - `{TITLE}`
  - `{DATE}` (original front‑matter date)
  - `{CONTENT}` (truncated)
  - `{PATH}` (relative link to generated post HTML)

Expansion logic in [`html::substitute_feed`](src/html.rs).  
Content truncation in [`markdown::truncate_content`](src/markdown.rs).

Example component (`src/components/post.html`):

```html
<article class="post">
  <h2><a href="{PATH}">{TITLE}</a></h2>
  <time>{DATE}</time>
  <p>{CONTENT}</p>
</article>
```

---

## Posts

Created by [`markdown::create_post`](src/markdown.rs). Generates front matter:

```yaml
---
title: Example Post
date: Tuesday 16 September 2025
---
```

Date parsing in [`markdown::parse_date`](src/markdown.rs) accepts:

- $%A\ %e\ %B\ %Y$ or
- $%e\ %B\ %Y$

Rendered to HTML via Pandoc in [`markdown::render_to_html`](src/markdown.rs) with `--mathjax`.

---

## Metadata & RSS

Per‑post Open Graph meta added by [`markdown::add_meta_to_post_html`](src/markdown.rs).  
Site‑wide RSS `<link rel="alternate"...>` injected by [`rss::add_rss_meta`](src/rss.rs).  
RSS feed assembled in Build via [`rss_gen`](Cargo.toml) producing `static/feed.xml`.

Each item uses:

- Title: post title
- Description: truncated body (`truncate_content`)
- GUID/Link: absolute URL
- PubDate: UTC midnight of post date

---

## Build Pipeline (Simplified)

1. Walk `src/`
2. For `.md`: convert -> inject meta -> write `.html`
3. For `.html`: expand `<REPLACE>` + `<FEED>` -> inject RSS link
4. Copy other assets
5. Emit `feed.xml`

Core functions:

- [`markdown::get_mdinfos_for_path`](src/markdown.rs)
- [`html::generate_substituted_html`](src/html.rs)
- [`rss::add_rss_meta`](src/rss.rs)

---

## Adding a New Component

1. Create `src/components/card.html`
2. Use it:
   ```html
   <REPLACE with="card.html" />
   ```
3. Rebuild.

---

## Creating a Post Without Opening Editor

```bash
static_site_generator Post "Draft Post" --open-in-editor false .
```

---

## Requirements

- Rust (edition 2024)
- Pandoc
- (Optional) $EDITOR env var for auto-open

---

## Notes / Limitations

- Components not nested (single directory)
- No incremental rebuild
- Date time is naive (midnight UTC assigned on RSS export)
- Basic error handling (uses [`thiserror`](Cargo.toml))

---

## Ideas / Future

- Pagination for feeds
- Nested component directories
- Asset hashing
- Live preview server
- Tests

---
