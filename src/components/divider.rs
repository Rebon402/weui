use leptos::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DividerOrientation {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DividerStrokeStyle {
    Solid,
    Dashed,
    Dotted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextAlign {
    Left,
    Center,
    Right,
}

#[component]
pub fn Divider(
    #[prop(into)] orientation: MaybeSignal<DividerOrientation>,
    #[prop(into)] stroke_style: MaybeSignal<DividerStrokeStyle>,
    #[prop(into)] align: MaybeSignal<TextAlign>,
    #[prop(into, default = "".into())] text: MaybeSignal<String>,
    #[prop(into, default = "".into())] class: MaybeSignal<String>,
    #[prop(into, default = "".into())] style: MaybeSignal<String>,
) -> impl IntoView {
    let orientation_class = move || match orientation.get() {
        DividerOrientation::Horizontal => "weui-divider--horizontal",
        DividerOrientation::Vertical => "weui-divider--vertical",
    };
    let stroke_class = move || match stroke_style.get() {
        DividerStrokeStyle::Solid => "",
        DividerStrokeStyle::Dashed => "weui-divider--dashed",
        DividerStrokeStyle::Dotted => "weui-divider--dotted",
    };
    let align_class = move || match align.get() {
        TextAlign::Left => "weui-divider--left",
        TextAlign::Center => "",
        TextAlign::Right => "weui-divider--right",
    };
    let has_text = move || !text.get().is_empty();
    view! {
        <div
            class=move || format!("weui-divider {} {} {} {}", orientation_class(), stroke_class(), align_class(), class.get())
            class=("weui-divider--with-text", has_text)
            role="separator"
            style=style
        >
            {move || {
                if has_text() {
                    view! { <span class="weui-divider__text">{text.get()}</span> }
                } else {
                    view! {}
                }
            }}
        </div>
    }
}
