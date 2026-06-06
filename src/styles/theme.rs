//! Embedded WeUI CSS helpers and style assets.

pub const DEFAULT_THEME: &str = "light";
pub const THEME_CSS: &str = include_str!("../../assets/theme.css");

/// Returns static metadata about the styles module.
pub fn theme_info() -> &'static str {
    "WeUI styles module"
}

/// Return the embedded WeUI stylesheet contents.
pub fn theme_css() -> &'static str {
    THEME_CSS
}

/// Return the embedded WeUI stylesheet as an inline `<style>` tag.
pub fn theme_style_tag() -> String {
    format!("<style>{}</style>", THEME_CSS)
}

/// Create a `<link>` tag to include the embedded stylesheet from an external path.
pub fn theme_stylesheet_link(path: &str) -> String {
    format!(r#"<link rel=\"stylesheet\" href=\"{}\" />"#, path)
}
