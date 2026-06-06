use leptos::*;
use leptos::Show;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NavbarVariant {
    Default,
    Primary,
    Transparent,
}

#[component]
pub fn Navbar(
    #[prop(into)] variant: MaybeSignal<NavbarVariant>,
    #[prop(into, default = "".into())] title: MaybeSignal<String>,
    #[prop(into, default = false.into())] show_back: MaybeSignal<bool>,
    #[prop(into, default = "".into())] back_text: MaybeSignal<String>,
    #[prop(into, default = false.into())] fixed: MaybeSignal<bool>,
    #[prop(into, default = "".into())] class: MaybeSignal<String>,
    #[prop(into, default = None.into())] on_back: Option<Callback<()>>,
    children: Children,
) -> impl IntoView {
    let class_clone = class.clone();
    let back_text_clone_for_condition = back_text.clone();
    let back_text_clone_for_content = back_text.clone();
    let title_clone_for_condition = title.clone();
    let title_clone_for_content = title.clone();
    let has_back_text = create_memo(move |_| !back_text_clone_for_condition.get().is_empty());
    let back_text_content = create_memo(move |_| back_text_clone_for_content.get());
    let has_title_text = create_memo(move |_| !title_clone_for_condition.get().is_empty());
    let title_content = create_memo(move |_| title_clone_for_content.get());
    let variant_class = move || match variant.get() {
        NavbarVariant::Default => "",
        NavbarVariant::Primary => "weui-navbar--primary",
        NavbarVariant::Transparent => "weui-navbar--transparent",
    };
    let variant_class_clone = variant_class.clone();
    view! {
        <nav
            class=move || format!("weui-navbar {} {}", variant_class_clone(), class_clone.get())
            class=("weui-navbar--fixed", move || fixed.get())
            role="navigation"
        >
            <Show when=show_back>
                <div class="weui-navbar__left">
                    <button
                        class="weui-navbar__back"
                        type="button"
                        aria-label="Go back"
                        on:click=move |_| { if let Some(cb) = &on_back { leptos::Callable::call(cb, ()) } }
                    >
                        <span class="weui-navbar__back-icon"/>
                        <Show when=move || has_back_text.get()>
                            <span>{back_text_content.get()}</span>
                        </Show>
                    </button>
                </div>
            </Show>
            <Show when=move || has_title_text.get()>
                <div class="weui-navbar__center">
                    <span class="weui-navbar__title">{title_content.get()}</span>
                </div>
            </Show>
            <div class="weui-navbar__right">
                {children()}
            </div>
        </nav>
    }
}
