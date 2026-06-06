# weui

WeUI is a Rust-first UI utility crate that helps you embed WeUI styles, generate HTML pages, and integrate with Leptos-based applications.

## Features

- Embedded WeUI CSS via `theme_css()` and `theme_style_tag()`
- HTML generation helpers with `html_page()` and `html_page_with_assets()`
- Leptos-friendly theme types and context utilities
- Example applications in `examples/html.rs` and `examples/http.rs`

## Quick start

```rust
use weui::{html_page, theme_css};

fn main() {
    let body = r#"<div class=\"weui-example\"><p>Hello WeUI!</p></div>"#;
    let html = html_page("WeUI Example", body, theme_css());
    println!("{}", html);
}
```

## Examples

- `cargo run --example html`
- `cargo run --example http`

## License

Apache-2.0
