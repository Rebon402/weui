use leptos::*;
use leptos::ssr::render_to_string;
use weui::{Button, ButtonShape, ButtonSize, ButtonType, ButtonVariant};
use weui::styles::theme_css;

#[component]
fn ExampleApp() -> impl IntoView {
    view! {
        <html lang="en">
            <head>
                <meta charset="UTF-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1.0"/>
                <title>"WeUI HTML Example"</title>
                <style>{theme_css()}</style>
            </head>
            <body>
                <div class="weui-example" style="padding: 24px;">
                    <h1>"WeUI Component Example"</h1>
                    <Button
                        text="Click me"
                        variant=ButtonVariant::Primary
                        size=ButtonSize::Normal
                        shape=ButtonShape::Round
                        button_type=ButtonType::Button
                    />
                </div>
            </body>
        </html>
    }
}

fn main() {
    let html = render_to_string(|| view! { <ExampleApp/> });
    println!("{}", html);
}
