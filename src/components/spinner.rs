use leptos::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpinnerType {
    Circular,
    Dots,
    Bars,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpinnerSize {
    Sm,
    Md,
    Lg,
}

#[component]
pub fn Spinner(
    #[prop(into)] spinner_type: MaybeSignal<SpinnerType>,
    #[prop(into)] size: MaybeSignal<SpinnerSize>,
    #[prop(into, default = "".into())] color: MaybeSignal<String>,
    #[prop(into, default = "".into())] class: MaybeSignal<String>,
) -> impl IntoView {
    let type_class = move || match spinner_type.get() {
        SpinnerType::Circular => "weui-spinner--circular",
        SpinnerType::Dots => "weui-spinner--dots",
        SpinnerType::Bars => "weui-spinner--bars",
    };
    let size_class = move || match size.get() {
        SpinnerSize::Sm => "weui-spinner--sm",
        SpinnerSize::Md => "",
        SpinnerSize::Lg => "weui-spinner--lg",
    };
    let spinner_style = move || {
        let c = color.get();
        if c.is_empty() {
            "".to_string()
        } else {
            format!("color: {};", c)
        }
    };
    view! {
        <div
            class=move || format!("weui-spinner {} {} {}", type_class(), size_class(), class.get())
            style=spinner_style
            role="status"
            aria-label="Loading"
        >
            {move || match spinner_type.get() {
                SpinnerType::Circular => view! {
                    <svg class="weui-spinner__circular" viewBox="25 25 50 50">
                        <circle cx="50" cy="50" r="20" fill="none" stroke="currentColor" stroke-width="4"/>
                    </svg>
                },
                SpinnerType::Dots => view! {
                    <div class="weui-spinner__dots">
                        <span class="weui-spinner__dot"/>
                        <span class="weui-spinner__dot"/>
                        <span class="weui-spinner__dot"/>
                    </div>
                },
                SpinnerType::Bars => view! {
                    <div class="weui-spinner__bars">
                        <span class="weui-spinner__bar"/>
                        <span class="weui-spinner__bar"/>
                        <span class="weui-spinner__bar"/>
                        <span class="weui-spinner__bar"/>
                    </div>
                },
            }}
        </div>
    }
}

#[component]
pub fn Loading(
    #[prop(into, default = false.into())] visible: MaybeSignal<bool>,
    #[prop(into, default = "".into())] text: MaybeSignal<String>,
    #[prop(into, default = false.into())] fullscreen: MaybeSignal<bool>,
    #[prop(into, default = SpinnerType::Circular.into())] spinner_type: MaybeSignal<SpinnerType>,
    #[prop(into, default = SpinnerSize::Md.into())] size: MaybeSignal<SpinnerSize>,
    #[prop(into, default = "".into())] class: MaybeSignal<String>,
) -> impl IntoView {
    view! {
        <Show when=move || visible.get()>
            <div
                class=move || format!("weui-loading {}", class.get())
                class:weui-loading--fullscreen=move || fullscreen.get()
            >
                <Spinner spinner_type=spinner_type size=size/>
                {move || {
                    if !text.get().is_empty() {
                        view! { <span class="weui-loading__text">{text.get()}</span> }
                    } else {
                        view! {}
                    }
                }}
            </div>
        </Show>
    }
}
