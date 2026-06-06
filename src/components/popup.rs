use leptos::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PopupPosition {
    Top,
    Bottom,
    Left,
    Right,
    Center,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PopupAnimation {
    Slide,
    Fade,
    None,
}

#[component]
pub fn Popup(
    #[prop(into)] visible: MaybeSignal<bool>,
    #[prop(into)] position: MaybeSignal<PopupPosition>,
    #[prop(into)] animation: MaybeSignal<PopupAnimation>,
    #[prop(into, default = true.into())] close_on_mask: MaybeSignal<bool>,
    #[prop(into, default = false.into())] round: MaybeSignal<bool>,
    #[prop(into, default = false.into())] safe_area: MaybeSignal<bool>,
    #[prop(into, default = "".into())] class: MaybeSignal<String>,
    #[prop(into, default = None.into())] on_close: Option<Callback<()>>,
    #[prop(into, default = None.into())] on_mask_click: Option<Callback<ev::MouseEvent>>,
    children: Children,
) -> impl IntoView {
    let position_class = move || match position.get() {
        PopupPosition::Top => "weui-popup--top",
        PopupPosition::Bottom => "weui-popup--bottom",
        PopupPosition::Left => "weui-popup--left",
        PopupPosition::Right => "weui-popup--right",
        PopupPosition::Center => "weui-popup--center",
    };
    let animation_class = move || match animation.get() {
        PopupAnimation::Slide => "weui-popup--slide",
        PopupAnimation::Fade => "weui-popup--fade",
        PopupAnimation::None => "",
    };
    let handle_mask_click = move |ev: ev::MouseEvent| {
        if let Some(cb) = &on_mask_click {
            cb.call(ev);
        }
        if close_on_mask.get() {
            if let Some(cb) = &on_close {
                cb.call(());
            }
        }
    };
    view! {
        <Show when=move || visible.get()>
            <div class="weui-popup__wrapper">
                <div
                    class="weui-popup__mask"
                    on:click=handle_mask_click
                />
                <div
                    class=move || format!("weui-popup {} {}", position_class(), animation_class(), class.get())
                    class:weui-popup--round=move || round.get()
                    class:weui-popup--safe-area=move || safe_area.get()
                    role="dialog"
                    aria-modal="true"
                >
                    {children()}
                </div>
            </div>
        </Show>
    }
}
