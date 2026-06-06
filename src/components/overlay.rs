use leptos::*;

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
    #[prop(into, default = None.into())] on_click: Option<Callback<ev::MouseEvent, ()>>,
    #[prop(into, default = None.into())] on_close: Option<Callback<(), ()>>,
) -> impl IntoView {
    let color_class = move || match color.get() {
        OverlayColor::Black => "weui-overlay--black",
        OverlayColor::White => "weui-overlay--white",
        OverlayColor::Transparent => "weui-overlay--transparent",
    };
    let handle_click = move |ev: ev::MouseEvent| {
        if let Some(cb) = &on_click {
            cb.call(ev);
        }
        if close_on_click.get() {
            if let Some(cb) = &on_close {
                cb.call(());
            }
        }
    };
    let overlay_style = move || {
        format!(
            "opacity: {}; z-index: {};",
            opacity.get(),
            z_index.get()
        )
    };
    view! {
        <Show when=move || visible.get()>
            <div
                class=move || format!("weui-overlay {} {}", color_class(), class.get())
                class=("weui-overlay--blur", move || blur.get())
                style=overlay_style
                on:click=handle_click
                aria-hidden="true"
            />
        </Show>
    }
}
