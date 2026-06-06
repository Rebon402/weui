use leptos::*;

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
    children: ChildrenFn,
) -> impl IntoView {
    let variant_class = move || match variant.get() {
        NavbarVariant::Default => "",
        NavbarVariant::Primary => "weui-navbar--primary",
        NavbarVariant::Transparent => "weui-navbar--transparent",
    };
    let handle_back = move |_: ev::MouseEvent| {
        if let Some(cb) = &on_back {
            cb.call(());
        }
    };
    let left = create_memo(move |_| {
        if show_back.get() {
            view! {
                <div class="weui-navbar__left">
                    <button
                        class="weui-navbar__back"
                        on:click=handle_back
                        type="button"
                        aria-label="Go back"
                    >
                        <span class="weui-navbar__back-icon"/>
                        {move || {
                            if !back_text.get().is_empty() {
                                view! { <span>{back_text.get()}</span> }
                            } else {
                                view! {}
                            }
                        }}
                    </button>
                </div>
            }
        } else {
            view! {}
        }
    });
    let center = create_memo(move |_| {
        if !title.get().is_empty() {
            view! {
                <div class="weui-navbar__center">
                    <span class="weui-navbar__title">{title.get()}</span>
                </div>
            }
        } else {
            view! {}
        }
    });
    view! {
        <nav
            class=move || format!("weui-navbar {} {}", variant_class(), class.get())
            class:weui-navbar--fixed=move || fixed.get()
            role="navigation"
        >
            {left}
            {center}
            <div class="weui-navbar__right">
                {children()}
            </div>
        </nav>
    }
}
