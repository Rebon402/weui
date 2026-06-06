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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CellLabelAlign {
    Left,
    Center,
    Right,
}

#[component]
pub fn Cell(
    #[prop(into, default = "".into())] title: MaybeSignal<String>,
    #[prop(into, default = "".into())] value: MaybeSignal<String>,
    #[prop(into, default = "".into())] label: MaybeSignal<String>,
    #[prop(into, default = CellSize::Medium.into())] size: MaybeSignal<CellSize>,
    #[prop(into, default = CellAlign::Middle.into())] align: MaybeSignal<CellAlign>,
    #[prop(into, default = false.into())] border: MaybeSignal<bool>,
    #[prop(into, default = false.into())] required: MaybeSignal<bool>,
    #[prop(into, default = false.into())] center: MaybeSignal<bool>,
    #[prop(into, default = false.into())] url: MaybeSignal<bool>,
    #[prop(into, default = "".into())] link: MaybeSignal<String>,
    #[prop(into, default = false.into())] clickable: MaybeSignal<bool>,
    #[prop(into, default = false.into())] is_link: MaybeSignal<bool>,
    #[prop(into, default = "".into())] icon: MaybeSignal<String>,
    #[prop(into, default = "".into())] right_icon: MaybeSignal<String>,
    #[prop(into, default = "".into())] class: MaybeSignal<String>,
    #[prop(into, default = "".into())] style: MaybeSignal<String>,
    #[prop(into, default = None.into())] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(optional)] prefix: Option<View>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] suffix: Option<View>,
    #[prop(optional)] footer: Option<View>,
) -> impl IntoView {
    let has_icon = move || !icon.get().is_empty();
    let has_title = move || !title.get().is_empty();
    let has_value = move || !value.get().is_empty() || children.is_some();
    let has_label = move || !label.get().is_empty();
    let has_prefix = prefix.is_some();
    let has_right_icon = move || !right_icon.get().is_empty();
    let has_footer = footer.is_some();
    let is_access = move || clickable.get() || url.get() || is_link.get();
    let handle_click = move |ev: ev::MouseEvent| {
        if clickable.get() || is_link.get() {
            if let Some(cb) = &on_click {
                cb.call(ev);
            }
        }
    };
    let content = view! {
        {prefix.map(|p| view! { <div class="weui-cell__hd">{p}</div> })}
        {move || {
            if has_icon() {
                view! { <div class="weui-cell__hd"><Icon name=icon.get() size=Size::Md/></div> }
            } else {
                view! {}
            }
        }}
        <div class="weui-cell__bd">
            {move || {
                if has_title() {
                    view! {
                        <div class="weui-cell__title">
                            {move || {
                                if has_label() {
                                    view! {
                                        <span class="weui-cell__label">{label.get()}</span>
                                        <span>{title.get()}</span>
                                    }.into_view()
                                } else {
                                    title.get().into_view()
                                }
                            }}
                        </div>
                    }.into_view()
                } else {
                    view! {}.into_view()
                }
            }}
            {children.map(|c| c())}
        </div>
        {move || {
            if has_value() {
                view! {
                    <div class="weui-cell__ft">
                        {value.get()}
                    </div>
                }.into_view()
            } else {
                view! {}.into_view()
            }
        }}
        {move || {
            if has_right_icon() {
                view! { <div class="weui-cell__ft"><Icon name=right_icon.get()/></div> }
            } else {
                view! {}
            }
        }}
        {suffix.map(|s| view! { <div class="weui-cell__ft">{s}</div> })}
    };
    let cell_classes = move || {
        let mut classes = vec!["weui-cell".to_string()];
        if size.get() == CellSize::Large {
            classes.push("weui-cell--large".to_string());
        }
        if align.get() == CellAlign::Top {
            classes.push("weui-cell--align-top".to_string());
        }
        if align.get() == CellAlign::Bottom {
            classes.push("weui-cell--align-bottom".to_string());
        }
        if !border.get() {
            classes.push("weui-cell--no-border".to_string());
        }
        if required.get() {
            classes.push("weui-cell--required".to_string());
        }
        if center.get() {
            classes.push("weui-cell--center".to_string());
        }
        if is_access() {
            classes.push("weui-cell--access".to_string());
        }
        if has_footer() {
            classes.push("weui-cell--footer".to_string());
        };
        classes.push(class.get());
        classes.join(" ")
    };

    move || {
        if url.get() && !link.get().is_empty() {
            view! {
                <a
                    class=cell_classes
                    style=style.get()
                    href=link.get()
                    role="button"
                    aria-label=title.get()
                >
                    {content}
                </a>
            }.into_view()
        } else {
            view! {
                <div
                    class=cell_classes
                    style=style.get()
                    on:click=handle_click
                    role=move || if is_access() { "button" } else { "listitem" }
                    tabindex=move || if is_access() { "0" } else { "-1" }
                    aria-label=title.get()
                >
                    {content}
                    {footer.map(|f| view! { <div class="weui-cell__footer">{f}</div> })}
                </div>
            }.into_view()
        }
    }
}

#[component]
pub fn CellGroup(
    #[prop(into, default = "".into())] title: MaybeSignal<String>,
    #[prop(into, default = true.into())] inset: MaybeSignal<bool>,
    #[prop(into, default = "".into())] class: MaybeSignal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="weui-cells__group">
            {move || {
                if !title.get().is_empty() {
                    view! { <div class="weui-cells__title">{title.get()}</div> }
                } else {
                    view! {}
                }
            }}
            <div
                class="weui-cells"
                class:weui-cells--inset=move || inset.get()
                role="list"
            >
                {children()}
            </div>
        </div>
    }
}