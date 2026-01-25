# static_site_generator

Minimal Rust static site & blog generator with:

- Markdown -> HTML via Pandoc (with MathJax support)
- **No JavaScript**
- Component inclusion tags
- Automatic feed page generation
- Open Graph + RSS metadata
- RSS 2.0 feed (`feed.xml`)
---

## Quick Start

```bash
cargo install --path .
mkdir my_site
static_site_generator my_site init
cd my_site
static_site_generator . post "My First Post"
# edit generated src/posts/YY_MM_DD_my_first_post.md
static_site_generator . build
# output in ./static
```

Ensure `pandoc` is installed and on PATH.

For more usage information see [docs](./docs/ai-yap.md)