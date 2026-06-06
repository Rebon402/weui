use leptos::*;
use leptos::Show;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TabbarVariant {
    Default,
    Primary,
}

#[derive(Debug, Clone)]
pub struct TabbarItemData {
    pub key: String,
    pub title: String,
    pub icon: Option<String>,
    pub badge: Option<String>,
    pub dot: bool,
}

#[component]
pub fn Tabbar(
    #[prop(into)] _active_key: MaybeSignal<String>,
    #[prop(into)] _on_change: Callback<String>,
    #[prop(into)] variant: MaybeSignal<TabbarVariant>,
    #[prop(into, default = false.into())] fixed: MaybeSignal<bool>,
    #[prop(into, default = false.into())] safe_area: MaybeSignal<bool>,
    #[prop(into, default = "".into())] class: MaybeSignal<String>,
    children: Children,
) -> impl IntoView {
    let variant_class = move || match variant.get() {
        TabbarVariant::Default => "",
        TabbarVariant::Primary => "weui-tabbar--primary",
    };
    view! {
        <nav
            class=move || format!("weui-tabbar {} {}", variant_class(), class.get())
            class=("weui-tabbar--fixed", move || fixed.get())
            class=("weui-tabbar--safe-area", move || safe_area.get())
            role="tablist"
        >
            {children()}
        </nav>
    }
}

#[component]
pub fn TabbarItem(
    #[prop(into)] key: MaybeSignal<String>,
    #[prop(into)] title: MaybeSignal<String>,
    #[prop(into, default = "".into())] icon: MaybeSignal<String>,
    #[prop(into, default = "".into())] badge: MaybeSignal<String>,
    #[prop(into, default = false.into())] dot: MaybeSignal<bool>,
    #[prop(into, default = "".into())] class: MaybeSignal<String>,
    #[prop(into, default = None.into())] on_click: Option<Callback<String, ()>>,
) -> impl IntoView {
    let active = use_context::<ReadSignal<String>>();
    let key_clone = key.clone();
    let key_clone3 = key.clone();
    let is_active = create_memo(move |_| {
        if let Some(active_key) = active {
            active_key.get() == key_clone.get()
        } else {
            false
        }
    });
    let handle_click = move |_: ev::MouseEvent| {
        if let Some(cb) = &on_click {
            leptos::Callable::call(cb, key_clone3.get());
        }
    };
    let class_clone = class.clone();
    let icon_clone = icon.clone();
    let icon_clone2 = icon.clone();
    let dot_clone = dot.clone();
    let badge_clone = badge.clone();
    let badge_clone2 = badge.clone();
    let dot_clone2 = dot.clone();
    let title_clone = title.clone();
    view! {
        <button
            class=move || format!("weui-tabbar__item {}", class_clone.get())
            class=("weui-tabbar__item--active", move || is_active.get())
            on:click=handle_click
            type="button"
            role="tab"
            aria-selected=move || is_active.get().to_string()
        >
            <span class="weui-tabbar__icon">
                <Show when=move || !icon_clone.get().is_empty()>
                    <span class="weui-tabbar__icon-img">{icon_clone2.get()}</span>
                </Show>
                <Show when=move || dot_clone.get()>
                    <span class="weui-tabbar__dot"/>
                </Show>
                <Show when=move || !dot_clone2.get() && !badge_clone.get().is_empty()>
                    <span class="weui-tabbar__badge">{badge_clone2.get()}</span>
                </Show>
            </span>
            <span class="weui-tabbar__label">{title_clone}</span>
        </button>
    }
}
