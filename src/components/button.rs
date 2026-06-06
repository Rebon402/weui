use leptos::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ButtonVariant {
    Primary,
    Default,
    Warn,
    Plain,
    Mini,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ButtonType {
    Button,
    Submit,
    Reset,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ButtonSize {
    Normal,
    Medium,
    Small,
    Mini,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ButtonShape {
    Square,
    Circle,
    Round,
}

#[component]
pub fn Button(
    #[prop(into)] variant: MaybeSignal<ButtonVariant>,
    #[prop(into)] size: MaybeSignal<ButtonSize>,
    #[prop(into)] shape: MaybeSignal<ButtonShape>,
    #[prop(into)] button_type: MaybeSignal<ButtonType>,
    #[prop(into, default = false.into())] disabled: MaybeSignal<bool>,
    #[prop(into, default = false.into())] loading: MaybeSignal<bool>,
    #[prop(into, default = false.into())] plain: MaybeSignal<bool>,
    #[prop(into, default = false.into())] hairline: MaybeSignal<bool>,
    #[prop(into, default = false.into())] block: MaybeSignal<bool>,
    #[prop(into, default = false.into())] square: MaybeSignal<bool>,
    #[prop(into, default = false.into())] round: MaybeSignal<bool>,
    #[prop(into, default = false.into())] icon: MaybeSignal<Option<String>>,
    #[prop(into, default = "".into())] class: MaybeSignal<String>,
    #[prop(into, default = "".into())] style: MaybeSignal<String>,
    #[prop(into, default = None.into())] on_click: Option<Callback<ev::MouseEvent>>,
    children: Children,
) -> impl IntoView {
    let base_class = "weui-btn";
    let variant_class = move || match variant.get() {
        ButtonVariant::Primary => "weui-btn--primary",
        ButtonVariant::Default => "weui-btn--default",
        ButtonVariant::Warn => "weui-btn--warn",
        ButtonVariant::Plain => "weui-btn--plain",
        ButtonVariant::Mini => "weui-btn--mini",
    };
    let size_class = move || match size.get() {
        ButtonSize::Normal => "",
        ButtonSize::Medium => "weui-btn--medium",
        ButtonSize::Small => "weui-btn--small",
        ButtonSize::Mini => "weui-btn--mini",
    };
    let shape_class = move || match shape.get() {
        ButtonShape::Square => "weui-btn--square",
        ButtonShape::Circle => "weui-btn--circle",
        ButtonShape::Round => "weui-btn--round",
    };
    let state_class = move || {
        let mut classes = Vec::new();
        if disabled.get() {
            classes.push("weui-btn--disabled");
        }
        if loading.get() {
            classes.push("weui-btn--loading");
        }
        if plain.get() {
            classes.push("weui-btn--plain");
        }
        if hairline.get() {
            classes.push("weui-btn--hairline");
        }
        if block.get() {
            classes.push("weui-btn--block");
        }
        if square.get() {
            classes.push("weui-btn--square");
        }
        if round.get() {
            classes.push("weui-btn--round");
        }
        classes.join(" ")
    };
    let type_attr = move || match button_type.get() {
        ButtonType::Button => "button",
        ButtonType::Submit => "submit",
        ButtonType::Reset => "reset",
    };
    let handle_click = move |ev: ev::MouseEvent| {
        if disabled.get() || loading.get() {
            ev.prevent_default();
            return;
        }
        if let Some(cb) = &on_click {
            cb.call(ev);
        }
    };
    view! {
        <button
            class=move || format!("{base_class} {variant_class} {size_class} {shape_class} {state_class} {}", class.get())
            class:weui-btn=true
            class:weui-btn--primary=move || variant.get() == ButtonVariant::Primary
            class:weui-btn--default=move || variant.get() == ButtonVariant::Default
            class:weui-btn--warn=move || variant.get() == ButtonVariant::Warn
            class:weui-btn--plain=move || plain.get()
            class:weui-btn--disabled=move || disabled.get()
            class:weui-btn--loading=move || loading.get()
            class:weui-btn--block=move || block.get()
            class:weui-btn--hairline=move || hairline.get()
            type=type_attr
            disabled=move || disabled.get()
            style=move || style.get()
            on:click=handle_click
            role="button"
            aria-disabled=move || disabled.get().to_string()
            aria-busy=move || loading.get().to_string()
        >
            {move || {
                if loading.get() {
                    view! { <span class="weui-btn__loading"><span class="weui-loading"/></span> }
                } else if let Some(icon_name) = icon.get() {
                    view! { <span class="weui-btn__icon"><Icon name=icon_name/></span> }
                } else {
                    view! {}
                }
            }}
            <span class="weui-btn__text">{children()}</span>
        </button>
    }
}

#[component]
pub fn ButtonGroup(
    #[prop(into, default = ButtonSize::Normal.into())] size: MaybeSignal<ButtonSize>,
    #[prop(into, default = false.into())] round: MaybeSignal<bool>,
    #[prop(into, default = false.into())] block: MaybeSignal<bool>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            class="weui-btn-group"
            class:weui-btn-group--round=move || round.get()
            class:weui-btn-group--block=move || block.get()
            role="group"
        >
            {children()}
        </div>
    }
}