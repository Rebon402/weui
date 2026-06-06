use weui::{html_page, html_page_with_assets, theme_css};

fn main() {
    let body = r#"
    <div class=\"weui-example\" style=\"padding: 24px;\">
        <h1>WeUI HTML Example</h1>
        <button class=\"weui-btn weui-btn--primary\">Click me</button>
        <div style=\"margin-top: 16px;\">
            <span class=\"weui-btn weui-btn--outline\">Secondary</span>
        </div>
    </div>
    "#;

    let inline_html = html_page("WeUI HTML Example", body, theme_css());
    println!("Inline CSS example:\n{}", inline_html);

    let external_html = html_page_with_assets(
        "WeUI HTML Example",
        body,
        Some("/style.css"),
        Some("/app.js"),
    );
    println!("External assets example:\n{}", external_html);
}
