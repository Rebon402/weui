# weui

[![docs.rs](https://img.shields.io/docsrs/weui)](https://docs.rs/weui) [![Crates.io](https://img.shields.io/crates/v/weui)](https://crates.io/crates/weui)

WeUI is a Rust-first UI utility crate for embedding WeUI styles, generating HTML pages, and integrating with Leptos applications.

## Features

- Embedded WeUI CSS via `theme_css()` and `theme_style_tag()`
- HTML generation helpers with `html_page()` and `html_page_with_assets()`
- Leptos-friendly theme types and context utilities
- Example applications in `examples/html.rs` and `examples/http.rs`
- Stable default build with optional `hydrate` / `nightly` support

## Quick start

```rust
use weui::{html_page, theme_css};

fn main() {
    let body = r#"<div class=\"weui-example\"><p>Hello WeUI!</p></div>"#;
    let html = html_page("WeUI Example", body, theme_css());
    println!("{}", html);
}
```

## Optional features

- `ssr` (default) — enable Leptos SSR utilities
- `hydrate` — enable Leptos hydration helpers
- `nightly` — enable Leptos nightly-only features

Build with optional features:

- `cargo build --features hydrate`
- `cargo build --features nightly`

## Examples

- `cargo run --example html`
- `cargo run --example http`

## License

Apache-2.0
