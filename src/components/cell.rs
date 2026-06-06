use super::icon::Icon;
use crate::theme::Size;
use super::icon::IconName;
use leptos::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CellSize {
    Medium,
    Large,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CellAlign {
    Top,
    Middle,
    Bottom,
}

#[component]
pub fn Cell(
    #[prop(into, default = "".into())] title: MaybeSignal<String>,
    #[prop(into, default = "".into())] value: MaybeSignal<String>,
    #[prop(into, default = "".into())] desc: MaybeSignal<String>,
    #[prop(into, default = CellSize::Medium.into())] size: MaybeSignal<CellSize>,
    #[prop(into, default = false.into())] border: MaybeSignal<bool>,
    #[prop(into, default = false.into())] required: MaybeSignal<bool>,
    #[prop(into, default = false.into())] center: MaybeSignal<bool>,
    #[prop(into, default = false.into())] clickable: MaybeSignal<bool>,
    #[prop(into, default = false.into())] is_link: MaybeSignal<bool>,
    #[prop(into, default = "".into())] icon: MaybeSignal<String>,
    #[prop(into, default = "".into())] right_icon: MaybeSignal<String>,
    #[prop(into, default = "".into())] url: MaybeSignal<String>,
) -> impl IntoView {
    view! {
        <div
            class="weui-cell"
            class=("weui-cell--large", move || size.get() == CellSize::Large)
            class=("weui-cell--center", move || center.get())
            class=("weui-cell--clickable", move || clickable.get())
            class=("weui-cell--access", move || is_link.get())
            class=("weui-cell--no-border", move || !border.get())
        >
            {move || {
                if !icon.get().is_empty() {
                    view! { <div class="weui-cell__hd"><Icon name=IconName::Info size=Size::Md/></div> }
                } else {
                    view! {}
                }
            }}
            <div class="weui-cell__bd">
                <div class="weui-cell__title">{title}</div>
                <div class="weui-cell__desc">{desc}</div>
            </div>
            <div class="weui-cell__ft">{value}</div>
            {move || {
                if !right_icon.get().is_empty() {
                    view! { <div class="weui-cell__ft"><Icon name=IconName::Arrow/></div> }
                } else {
                    view! {}
                }
            }}
        </div>
    }
}

#[component]
pub fn CellGroup(
    #[prop(into, default = "".into())] title: MaybeSignal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="weui-cells-wrap">
            {move || {
                if !title.get().is_empty() {
                    view! { <div class="weui-cells__title">{title.get()}</div> }
                } else {
                    view! {}
                }
            }}
            <div class="weui-cells" role="list">
                {children()}
            </div>
        </div>
    }
}
