/// Generate a complete HTML document with inline CSS.
///
/// # Examples
///
/// ```rust
/// use weui::{html_page, theme_css};
///
/// let document = html_page("Hello", "<p>Hi</p>", theme_css());
/// assert!(document.contains("<style>"));
/// ```
pub fn html_page(title: &str, body: &str, css: &str) -> String {
    format!(r#"<!DOCTYPE html>
<html lang=\"en\">
<head>
    <meta charset=\"UTF-8\" />
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\" />
    <title>{}</title>
    <style>{}</style>
</head>
<body>
{}
</body>
</html>"#, title, css, body)
}

/// Generate a complete HTML document using optional external assets.
///
/// If `css_path` is provided, a stylesheet link is added. If `js_path` is provided,
/// a deferred script tag is added.
///
/// # Examples
///
/// ```rust
/// use weui::html_page_with_assets;
///
/// let document = html_page_with_assets(
///     "Hello",
///     "<p>Hi</p>",
///     Some("/style.css"),
///     Some("/app.js"),
/// );
/// assert!(document.contains("/style.css"));
/// assert!(document.contains("/app.js"));
/// ```
pub fn html_page_with_assets(
    title: &str,
    body: &str,
    css_path: Option<&str>,
    js_path: Option<&str>,
) -> String {
    let stylesheet = css_path
        .map(|path| format!(r#"    <link rel=\"stylesheet\" href=\"{}\" />
"#, path))
        .unwrap_or_default();

    let script = js_path
        .map(|path| format!(r#"    <script defer src=\"{}\"></script>
"#, path))
        .unwrap_or_default();

    format!(r#"<!DOCTYPE html>
<html lang=\"en\">
<head>
    <meta charset=\"UTF-8\" />
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\" />
    <title>{}</title>
{}{}
</head>
<body>
{}
</body>
</html>"#, title, stylesheet, script, body)
}
