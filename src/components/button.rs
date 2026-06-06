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
    #[prop(into, default = "".into())] text: MaybeSignal<String>,
    #[prop(into, default = ButtonVariant::Primary.into())] variant: MaybeSignal<ButtonVariant>,
    #[prop(into, default = ButtonSize::Normal.into())] size: MaybeSignal<ButtonSize>,
    #[prop(into, default = ButtonShape::Round.into())] shape: MaybeSignal<ButtonShape>,
    #[prop(into, default = ButtonType::Button.into())] button_type: MaybeSignal<ButtonType>,
    #[prop(into, default = false.into())] disabled: MaybeSignal<bool>,
    #[prop(into, default = false.into())] loading: MaybeSignal<bool>,
    #[prop(into, default = false.into())] plain: MaybeSignal<bool>,
    #[prop(into, default = false.into())] hairline: MaybeSignal<bool>,
    #[prop(into, default = false.into())] block: MaybeSignal<bool>,
    #[prop(into, default = None.into())] on_click: Option<Callback<ev::MouseEvent, ()>>,
) -> impl IntoView {
    let handle_click = move |ev: ev::MouseEvent| {
        if let Some(cb) = &on_click {
            cb.call(ev);
        }
    };
    view! {
        <button
            class="weui-btn"
            class=("weui-btn--primary", move || variant.get() == ButtonVariant::Primary)
            class=("weui-btn--default", move || variant.get() == ButtonVariant::Default)
            class=("weui-btn--warn", move || variant.get() == ButtonVariant::Warn)
            class=("weui-btn--plain", move || plain.get())
            class=("weui-btn--disabled", move || disabled.get())
            class=("weui-btn--loading", move || loading.get())
            class=("weui-btn--block", move || block.get())
            class=("weui-btn--hairline", move || hairline.get())
            class=("weui-btn--round", move || shape.get() == ButtonShape::Round)
            class=("weui-btn--circle", move || shape.get() == ButtonShape::Circle)
            class=("weui-btn--square", move || shape.get() == ButtonShape::Square)
            class=("weui-btn--small", move || size.get() == ButtonSize::Small || size.get() == ButtonSize::Mini)
            type=move || match button_type.get() { ButtonType::Button => "button", ButtonType::Submit => "submit", ButtonType::Reset => "reset" }
            disabled=move || disabled.get()
            role="button"
            aria-disabled=move || disabled.get().to_string()
            aria-busy=move || loading.get().to_string()
            on:click=handle_click
        >
            <Show when=move || loading.get()>
                <span class="weui-loading"/>
            </Show>
            <span class="weui-btn__text">{text}</span>
        </button>
    }
}

#[component]
pub fn ButtonGroup(children: Children) -> impl IntoView {
    view! {
        <div class="weui-btn-group" role="group">
            {children()}
        </div>
    }
}
