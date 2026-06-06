use weui::{html_page, html_page_with_assets, theme_css};

#[test]
fn inline_html_page_contains_style_tag() {
    let html = html_page("Test", "<p>Body</p>", theme_css());
    assert!(html.contains("<style>"), "Expected inline HTML to include a style tag");
    assert!(html.contains("<p>Body</p>"), "Expected HTML body to be included");
}

#[test]
fn html_page_with_assets_contains_asset_paths() {
    let html = html_page_with_assets(
        "Asset Test",
        "<p>Body</p>",
        Some("/style.css"),
        Some("/app.js"),
    );

    assert!(html.contains("/style.css"), "Expected stylesheet path to be included");
    assert!(html.contains("/app.js"), "Expected script path to be included");
    assert!(html.contains("<body>"), "Expected HTML body wrapper");
}
