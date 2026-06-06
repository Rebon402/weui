use leptos::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TagType {
    Default,
    Primary,
    Success,
    Warning,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TagSize {
    Sm,
    Md,
    Lg,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TagVariant {
    Dark,
    Light,
    Outlined,
    Plain,
}

#[component]
pub fn Tag(
    #[prop(into)] tag_type: MaybeSignal<TagType>,
    #[prop(into)] size: MaybeSignal<TagSize>,
    #[prop(into)] variant: MaybeSignal<TagVariant>,
    #[prop(into, default = false.into())] closable: MaybeSignal<bool>,
    #[prop(into, default = false.into())] round: MaybeSignal<bool>,
    #[prop(into, default = false.into())] mark: MaybeSignal<bool>,
    #[prop(into, default = false.into())] visible: MaybeSignal<bool>,
    #[prop(into, default = "".into())] class: MaybeSignal<String>,
    #[prop(into, default = None.into())] on_close: Option<Callback<(), ()>>,
    children: Children,
) -> impl IntoView {
    let type_class = move || match tag_type.get() {
        TagType::Default => "",
        TagType::Primary => "weui-tag--primary",
        TagType::Success => "weui-tag--success",
        TagType::Warning => "weui-tag--warning",
        TagType::Error => "weui-tag--error",
    };
    let size_class = move || match size.get() {
        TagSize::Sm => "weui-tag--sm",
        TagSize::Md => "",
        TagSize::Lg => "weui-tag--lg",
    };
    let variant_class = move || match variant.get() {
        TagVariant::Dark => "",
        TagVariant::Light => "weui-tag--light",
        TagVariant::Outlined => "weui-tag--outlined",
        TagVariant::Plain => "weui-tag--plain",
    };
    let handle_close = move |ev: ev::MouseEvent| {
        ev.stop_propagation();
        if let Some(cb) = &on_close {
            cb.call(());
        }
    };
    view! {
        <Show when=move || visible.get()>
            <span class=|| format!("weui-tag {} {} {} {}", type_class(), size_class(), variant_class(), class.get())
                class=("weui-tag--round", move || round.get())
                class=("weui-tag--mark", move || mark.get())
            >
                {move || children()}
                <Show when=move || closable.get()>
                    <button
                        class="weui-tag__close"
                        on:click=handle_close
                        type="button"
                        aria-label="Close tag"
                    />
                </Show>
            </span>
        </Show>
    }
}
