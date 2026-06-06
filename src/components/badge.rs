use leptos::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BadgeType {
    Dot,
    Count,
    Text,
    Corner,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BadgeSize {
    Sm,
    Md,
    Lg,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BadgeVariant {
    Default,
    Primary,
    Success,
    Warning,
    Error,
}

#[component]
pub fn Badge(
    #[prop(into)] badge_type: MaybeSignal<BadgeType>,
    #[prop(into)] size: MaybeSignal<BadgeSize>,
    #[prop(into)] variant: MaybeSignal<BadgeVariant>,
    #[prop(into, default = "".into())] text: MaybeSignal<String>,
    #[prop(into, default = 0.into())] count: MaybeSignal<i32>,
    #[prop(into, default = false.into())] visible: MaybeSignal<bool>,
    #[prop(into, default = "".into())] class: MaybeSignal<String>,
    children: Children,
) -> impl IntoView {
    let is_dot = move || badge_type.get() == BadgeType::Dot;
    let content = move || match badge_type.get() {
        BadgeType::Dot => "",
        BadgeType::Count => &format!("{}", count.get()),
        BadgeType::Text => text.get().as_str(),
        BadgeType::Corner => text.get().as_str(),
    };
    view! {
        <span
            class=move || format!("weui-badge {} {} {} {}", class.get(), 
                if badge_type.get() == BadgeType::Dot { "weui-badge--dot" } else { "" },
                if visible.get() { "weui-badge--visible" } else { "" },
                match size.get() { BadgeSize::Sm => "weui-badge--sm", BadgeSize::Md => "", BadgeSize::Lg => "weui-badge--lg" })
            class=("weui-badge--default", move || variant.get() == BadgeVariant::Default)
            class=("weui-badge--primary", move || variant.get() == BadgeVariant::Primary)
            class=("weui-badge--success", move || variant.get() == BadgeVariant::Success)
            class=("weui-badge--warning", move || variant.get() == BadgeVariant::Warning)
            class=("weui-badge--error", move || variant.get() == BadgeVariant::Error)
        >
            {move || if is_dot() { view! {} } else { view! { <span class="weui-badge__content">{content()}</span> }} }
            <span class="weui-badge__wrapper">
                {children()}
            </span>
        </span>
    }
}