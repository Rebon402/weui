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
    let badge_content = move || match badge_type.get() {
        BadgeType::Dot => None,
        BadgeType::Count => Some(format!("{}", count.get())),
        BadgeType::Text => Some(text.get()),
        BadgeType::Corner => Some(text.get()),
    };
    let variant_class = move || match variant.get() {
        BadgeVariant::Default => "weui-badge--default",
        BadgeVariant::Primary => "weui-badge--primary",
        BadgeVariant::Success => "weui-badge--success",
        BadgeVariant::Warning => "weui-badge--warning",
        BadgeVariant::Error => "weui-badge--error",
    };
    let size_class = move || match size.get() {
        BadgeSize::Sm => "weui-badge--sm",
        BadgeSize::Md => "",
        BadgeSize::Lg => "weui-badge--lg",
    };
    view! {
        <span
            class=move || format!("weui-badge {}", class.get())
            class:weui-badge--dot=move || badge_type.get() == BadgeType::Dot
            class:weui-badge--visible=move || visible.get()
            class:variant_class=move || true
            class:size_class=move || true
        >
            {badge_content}
            <span class="weui-badge__wrapper">
                {children()}
            </span>
        </span>
    }
}
