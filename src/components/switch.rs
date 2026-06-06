use leptos::*;
use leptos::Show;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SwitchSize {
    Sm,
    Md,
    Lg,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SwitchVariant {
    Default,
    Round,
    Square,
}

#[component]
pub fn Switch(
    #[prop(into)] checked: MaybeSignal<bool>,
    #[prop(into)] on_change: Callback<bool>,
    #[prop(into)] size: MaybeSignal<SwitchSize>,
    #[prop(into)] variant: MaybeSignal<SwitchVariant>,
    #[prop(into, default = false.into())] disabled: MaybeSignal<bool>,
    #[prop(into, default = false.into())] loading: MaybeSignal<bool>,
    #[prop(into, default = "".into())] active_text: MaybeSignal<String>,
    #[prop(into, default = "".into())] inactive_text: MaybeSignal<String>,
    #[prop(into, default = "".into())] class: MaybeSignal<String>,
    #[prop(into, default = None.into())] on_click: Option<Callback<ev::MouseEvent, ()>>,
) -> impl IntoView {
    let size_class = move || match size.get() {
        SwitchSize::Sm => "weui-switch--sm",
        SwitchSize::Md => "",
        SwitchSize::Lg => "weui-switch--lg",
    };
    let variant_class = move || match variant.get() {
        SwitchVariant::Default => "",
        SwitchVariant::Round => "weui-switch--round",
        SwitchVariant::Square => "weui-switch--square",
    };
    let handle_click = move |ev: ev::MouseEvent| {
        if disabled.get() || loading.get() {
            ev.prevent_default();
            return;
        }
        if let Some(cb) = &on_click {
            cb.call(ev);
        }
        on_change.call(!checked.get());
    };
    view! {
        <button
            class=move || format!("weui-switch {} {} {}", size_class(), variant_class(), class.get())
            class=("weui-switch--checked", move || checked.get())
            class=("weui-switch--disabled", move || disabled.get())
            class=("weui-switch--loading", move || loading.get())
            on:click=handle_click
            type="button"
            role="switch"
            aria-checked=move || checked.get().to_string()
            aria-disabled=move || disabled.get().to_string()
            disabled=move || disabled.get()
        >
            <span class="weui-switch__handle">
                <Show when=move || loading.get()>
                    <span class="weui-switch__loading"/>
                </Show>
            </span>
            <Show when=move || !active_text.get().is_empty() || !inactive_text.get().is_empty()>
                <span class="weui-switch__label">
                    <Show when=move || checked.get()>
                        <span>{active_text.get()}</span>
                    </Show>
                    <Show when=move || !checked.get()>
                        <span>{inactive_text.get()}</span>
                    </Show>
                </span>
            </Show>
        </button>
    }
}

#[component]
pub fn Toggle(
    #[prop(into)] checked: MaybeSignal<bool>,
    #[prop(into)] on_change: Callback<bool>,
    #[prop(into, default = false.into())] disabled: MaybeSignal<bool>,
    #[prop(into, default = "".into())] class: MaybeSignal<String>,
) -> impl IntoView {
    view! {
        <Switch
            checked=checked
            on_change=on_change
            size=SwitchSize::Md
            variant=SwitchVariant::Default
            disabled=disabled
            class=class
        />
    }
}
