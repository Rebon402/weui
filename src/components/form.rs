use leptos::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FormLayout {
    Horizontal,
    Vertical,
    Inline,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValidateStatus {
    Success,
    Warning,
    Error,
    Validating,
}

#[derive(Debug, Clone)]
pub struct FormFieldState {
    pub value: String,
    pub error: Option<String>,
    pub status: ValidateStatus,
    pub touched: bool,
}

#[derive(Clone)]
pub struct FormState {
    pub fields: RwSignal<std::collections::HashMap<String, FormFieldState>>,
}

impl FormState {
    pub fn new() -> Self {
        Self {
            fields: RwSignal::new(std::collections::HashMap::new()),
        }
    }
}

#[component]
pub fn Form(
    #[prop(into)] layout: MaybeSignal<FormLayout>,
    #[prop(into, default = false.into())] disabled: MaybeSignal<bool>,
    #[prop(into, default = false.into())] readonly: MaybeSignal<bool>,
    #[prop(into, default = "".into())] class: MaybeSignal<String>,
    children: Children,
) -> impl IntoView {
    let form_state = FormState::new();
    provide_context(form_state);
    let layout_class = move || match layout.get() {
        FormLayout::Horizontal => "weui-form--horizontal",
        FormLayout::Vertical => "",
        FormLayout::Inline => "weui-form--inline",
    };
    view! {
        <form
            class=move || format!("weui-form {} {}", layout_class(), class.get())
            class:weui-form--disabled=move || disabled.get()
            class:weui-form--readonly=move || readonly.get()
            novalidate="true"
        >
            {children()}
        </form>
    }
}

#[component]
pub fn FormField(
    #[prop(into)] name: MaybeSignal<String>,
    #[prop(into, default = "".into())] label: MaybeSignal<String>,
    #[prop(into, default = false.into())] required: MaybeSignal<bool>,
    #[prop(into, default = ValidateStatus::Success.into())] status: MaybeSignal<ValidateStatus>,
    #[prop(into, default = "".into())] error_message: MaybeSignal<String>,
    #[prop(into, default = "".into())] help_text: MaybeSignal<String>,
    #[prop(into, default = "".into())] class: MaybeSignal<String>,
    children: Children,
) -> impl IntoView {
    let status_class = move || match status.get() {
        ValidateStatus::Success => "",
        ValidateStatus::Warning => "weui-form-field--warning",
        ValidateStatus::Error => "weui-form-field--error",
        ValidateStatus::Validating => "weui-form-field--validating",
    };
    view! {
        <div
            class=move || format!("weui-form-field {} {}", status_class(), class.get())
            class:weui-form-field--required=move || required.get()
        >
            {move || {
                if !label.get().is_empty() {
                    view! {
                        <label class="weui-form-field__label">
                            {label.get()}
                            {move || {
                                if required.get() {
                                    view! { <span class="weui-form-field__required">*</span> }
                                } else {
                                    view! {}
                                }
                            }}
                        </label>
                    }
                } else {
                    view! {}
                }
            }}
            <div class="weui-form-field__control">
                {children()}
            </div>
            {move || {
                if !error_message.get().is_empty() {
                    view! { <div class="weui-form-field__error">{error_message.get()}</div> }
                } else if !help_text.get().is_empty() {
                    view! { <div class="weui-form-field__help">{help_text.get()}</div> }
                } else {
                    view! {}
                }
            }}
        </div>
    }
}
