use leptos::*;
use leptos::Show;

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
pub enum DividerColor {
    Default,
    Primary,
    Success,
    Warning,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextAlign {
    Left,
    Center,
    Right,
}

#[component]
pub fn Divider(
    #[prop(into, default = "".into())] text: MaybeSignal<String>,
    #[prop(into, default = DividerOrientation::Horizontal.into())] orientation: MaybeSignal<DividerOrientation>,
    #[prop(into, default = DividerStrokeStyle::Solid.into())] stroke: MaybeSignal<DividerStrokeStyle>,
    #[prop(into, default = DividerColor::Default.into())] color: MaybeSignal<DividerColor>,
    #[prop(into, default = TextAlign::Center.into())] align: MaybeSignal<TextAlign>,
    #[prop(into, default = "".into())] class: MaybeSignal<String>,
    #[prop(into, default = "".into())] style: MaybeSignal<String>,
) -> impl IntoView {
    let orientation_class = move || match orientation.get() {
        DividerOrientation::Horizontal => "weui-divider--horizontal",
        DividerOrientation::Vertical => "weui-divider--vertical",
    };
    let stroke_class = move || match stroke.get() {
        DividerStrokeStyle::Solid => "weui-divider--solid",
        DividerStrokeStyle::Dashed => "weui-divider--dashed",
        DividerStrokeStyle::Dotted => "weui-divider--dotted",
    };
    let color_class = move || match color.get() {
        DividerColor::Default => "",
        DividerColor::Primary => "weui-divider--primary",
        DividerColor::Success => "weui-divider--success",
        DividerColor::Warning => "weui-divider--warning",
        DividerColor::Error => "weui-divider--error",
    };
    let align_class = move || match align.get() {
        TextAlign::Left => "weui-divider--left",
        TextAlign::Center => "",
        TextAlign::Right => "weui-divider--right",
    };
    let text_clone1 = text.clone();
    let text_clone2 = text.clone();
    let text_clone3 = text.clone();
    view! {
        <div
            class=move || format!("weui-divider {} {} {} {} {}", orientation_class(), stroke_class(), align_class(), color_class(), class.get())
            class=("weui-divider--with-text", move || !text_clone1.get().is_empty())
            role="separator"
            style=style
        >
            <Show when=move || !text_clone2.get().is_empty()>
                <span class="weui-divider__text">{text_clone3.get()}</span>
            </Show>
        </div>
    }
}
