use leptos::*;
use leptos::Show;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OverlayColor {
    Black,
    White,
    Transparent,
}

#[component]
pub fn Overlay(
    #[prop(into)] visible: MaybeSignal<bool>,
    #[prop(into, default = 0.5.into())] opacity: MaybeSignal<f64>,
    #[prop(into, default = 1000.into())] z_index: MaybeSignal<i32>,
    #[prop(into)] color: MaybeSignal<OverlayColor>,
    #[prop(into, default = true.into())] close_on_click: MaybeSignal<bool>,
    #[prop(into, default = false.into())] blur: MaybeSignal<bool>,
    #[prop(into, default = "".into())] class: MaybeSignal<String>,
) -> impl IntoView {
    let color_class = move || match color.get() {
        OverlayColor::Black => "weui-overlay--black",
        OverlayColor::White => "weui-overlay--white",
        OverlayColor::Transparent => "weui-overlay--transparent",
    };
    let class_clone = class.clone();
    view! {
        <Show when=move || visible.get()>
            <div
                class=move || format!("weui-overlay {} {}", color_class(), class_clone.get())
                class=("weui-overlay--blur", move || blur.get())
                style=move || format!("opacity: {}; z-index: {};", opacity.get(), z_index.get())
                aria-hidden="true"
            />
        </Show>
    }
}
