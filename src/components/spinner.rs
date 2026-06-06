use leptos::*;
use leptos::Show;

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
    let class_clone = class.clone();
    let spinner_type_clone = spinner_type.clone();
    view! {
        <div
            class=move || format!("weui-spinner {} {} {}", type_class(), size_class(), class_clone.get())
            style=spinner_style
            role="status"
            aria-label="Loading"
        >
            {move || {
                let content: View = match spinner_type_clone.get() {
                    SpinnerType::Circular => view! {
                        <svg class="weui-spinner__circular" viewBox="25 25 50 50">
                            <circle cx="50" cy="50" r="20" fill="none" stroke="currentColor" stroke-width="4"/>
                        </svg>
                    }.into_view(),
                    SpinnerType::Dots => view! {
                        <div class="weui-spinner__dots">
                            <span class="weui-spinner__dot"/>
                            <span class="weui-spinner__dot"/>
                            <span class="weui-spinner__dot"/>
                        </div>
                    }.into_view(),
                    SpinnerType::Bars => view! {
                        <div class="weui-spinner__bars">
                            <span class="weui-spinner__bar"/>
                            <span class="weui-spinner__bar"/>
                            <span class="weui-spinner__bar"/>
                            <span class="weui-spinner__bar"/>
                        </div>
                    }.into_view(),
                };
                content
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
    let class_clone2 = class.clone();
    let text_clone = text.clone();
    let text_clone_for_show = text_clone.clone();
    let text_clone_for_loading = text_clone.clone();
    let show_text = create_memo(move |_| !text_clone_for_show.get().is_empty());
    let loading_class = create_memo(move |_| format!("weui-loading {}", class_clone2.get()));
    let loading_text = create_memo(move |_| text_clone_for_loading.get());
    view! {
        <Show when=move || visible.get()>
            <div
                class=move || loading_class.get()
                class=("weui-loading--fullscreen", move || fullscreen.get())
            >
                <Spinner spinner_type=spinner_type size=size/>
                <Show when=move || show_text.get()>
                    <span class="weui-loading__text">
                        {loading_text.get()}
                    </span>
                </Show>
            </div>
        </Show>
    }
}
