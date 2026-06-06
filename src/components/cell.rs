use leptos::*;
use leptos::Show;

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
    #[prop(into, default = false.into())] _required: MaybeSignal<bool>,
    #[prop(into, default = false.into())] center: MaybeSignal<bool>,
    #[prop(into, default = false.into())] clickable: MaybeSignal<bool>,
    #[prop(into, default = false.into())] is_link: MaybeSignal<bool>,
    #[prop(into, default = "".into())] icon: MaybeSignal<String>,
    #[prop(into, default = "".into())] right_icon: MaybeSignal<String>,
    #[prop(into, default = "".into())] _url: MaybeSignal<String>,
) -> impl IntoView {
    let title_clone = title.clone();
    let icon_clone = icon.clone();
    let right_icon_clone = right_icon.clone();
    let value_clone = value.clone();
    let desc_clone = desc.clone();
    view! {
        <div
            class="weui-cell"
            class=("weui-cell--large", move || size.get() == CellSize::Large)
            class=("weui-cell--center", move || center.get())
            class=("weui-cell--clickable", move || clickable.get())
            class=("weui-cell--access", move || is_link.get())
            class=("weui-cell--no-border", move || !border.get())
        >
            <Show when=move || !icon_clone.get().is_empty()>
                <div class="weui-cell__hd"><super::icon::Icon name=super::icon::IconName::Info size=crate::theme::Size::Md/></div>
            </Show>
            <div class="weui-cell__bd">
                <div class="weui-cell__title">{title_clone}</div>
                <div class="weui-cell__desc">{desc_clone}</div>
            </div>
            <div class="weui-cell__ft">{value_clone}</div>
            <Show when=move || !right_icon_clone.get().is_empty()>
                <div class="weui-cell__ft"><super::icon::Icon name=super::icon::IconName::Arrow/></div>
            </Show>
        </div>
    }
}

#[component]
pub fn CellGroup(
    #[prop(into, default = "".into())] title: MaybeSignal<String>,
    children: Children,
) -> impl IntoView {
    let title_clone1 = title.clone();
    let title_clone2 = title.clone();
    view! {
        <div class="weui-cells-wrap">
            <Show when=move || !title_clone1.get().is_empty()>
                <div class="weui-cells__title">{title_clone2.get()}</div>
            </Show>
            <div class="weui-cells" role="list">
                {children()}
            </div>
        </div>
    }
}
