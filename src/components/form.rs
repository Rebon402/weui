use leptos::*;
use leptos::Show;

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

#[component]
pub fn Form(
    #[prop(into, default = FormLayout::Vertical.into())] layout: MaybeSignal<FormLayout>,
    children: Children,
) -> impl IntoView {
    let layout_class = move || match layout.get() {
        FormLayout::Horizontal => "weui-form--horizontal",
        FormLayout::Vertical => "weui-form--vertical",
        FormLayout::Inline => "weui-form--inline",
    };
    view! {
        <form class=layout_class role="form">
            {children()}
        </form>
    }
}

#[component]
pub fn FormField(
    #[prop(into, default = "".into())] label: MaybeSignal<String>,
    #[prop(into, default = false.into())] required: MaybeSignal<bool>,
    #[prop(into, default = "".into())] error_message: MaybeSignal<String>,
    #[prop(into, default = "".into())] help_text: MaybeSignal<String>,
    #[prop(into, default = ValidateStatus::Success.into())] status: MaybeSignal<ValidateStatus>,
    #[prop(into, default = "".into())] class: MaybeSignal<String>,
    children: Children,
) -> impl IntoView {
    let status_class = move || match status.get() {
        ValidateStatus::Success => "weui-form-field--success",
        ValidateStatus::Warning => "weui-form-field--warning",
        ValidateStatus::Error => "weui-form-field--error",
        ValidateStatus::Validating => "weui-form-field--validating",
    };
    let label_val1 = label.clone();
    let label_val2 = label.clone();
    let error_message_clone1 = error_message.clone();
    let error_message_clone2 = error_message.clone();
    let error_message_clone3 = error_message.clone();
    let help_text_clone1 = help_text.clone();
    let help_text_clone2 = help_text.clone();
    view! {
        <div
            class=move || format!("weui-form-field {} {}", status_class(), class.get())
            class=("weui-form-field--required", move || required.get())
        >
            <Show when=move || !label_val1.get().is_empty()>
                <label class="weui-form-field__label">
                    {label_val2.get()}
                    <Show when=move || required.get()>
                        <span class="weui-form-field__required">*</span>
                    </Show>
                </label>
            </Show>
            <div class="weui-form-field__control">
                {children()}
            </div>
            <Show when=move || !error_message_clone1.get().is_empty()>
                <div class="weui-form-field__error">{error_message_clone2.get()}</div>
            </Show>
            <Show when=move || error_message_clone3.get().is_empty() && !help_text_clone1.get().is_empty()>
                <div class="weui-form-field__help">{help_text_clone2.get()}</div>
            </Show>
        </div>
    }
}
