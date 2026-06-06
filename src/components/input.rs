use leptos::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputType {
    Text,
    Password,
    Number,
    Tel,
    Email,
    Url,
    Search,
    Date,
    Time,
    DatetimeLocal,
    Month,
    Week,
    Color,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputAlign {
    Left,
    Center,
    Right,
}

#[component]
pub fn Input(
    #[prop(into)] value: MaybeSignal<String>,
    #[prop(into)] on_input: Callback<String>,
    #[prop(into, default = InputType::Text.into())] input_type: MaybeSignal<InputType>,
    #[prop(into, default = "".into())] placeholder: MaybeSignal<String>,
    #[prop(into, default = false.into())] disabled: MaybeSignal<bool>,
    #[prop(into, default = false.into())] readonly: MaybeSignal<bool>,
    #[prop(into, default = false.into())] required: MaybeSignal<bool>,
    #[prop(into, default = false.into())] clearable: MaybeSignal<bool>,
    #[prop(into, default = false.into())] error: MaybeSignal<bool>,
    #[prop(into, default = false.into())] focus: MaybeSignal<bool>,
    #[prop(into, default = "".into())] label: MaybeSignal<String>,
    #[prop(into, default = "".into())] error_message: MaybeSignal<String>,
    #[prop(into, default = InputAlign::Left.into())] align: MaybeSignal<InputAlign>,
    #[prop(into, default = "".into())] class: MaybeSignal<String>,
    #[prop(into, default = "".into())] style: MaybeSignal<String>,
) -> impl IntoView {
    let input_ref = create_node_ref::<html::Input>();
    let has_value = create_memo(move |_| !value.get().is_empty());
    let show_clear = create_memo(move |_| clearable.get() && has_value.get() && !disabled.get());
    create_effect(move |_| {
        if focus.get() {
            input_ref.get().map(|el| el.focus().ok());
        }
    });
    let type_attr = move || -> &'static str {
        match input_type.get() {
            InputType::Text => "text",
            InputType::Password => "password",
            InputType::Number => "number",
            InputType::Tel => "tel",
            InputType::Email => "email",
            InputType::Url => "url",
            InputType::Search => "search",
            InputType::Date => "date",
            InputType::Time => "time",
            InputType::DatetimeLocal => "datetime-local",
            InputType::Month => "month",
            InputType::Week => "week",
            InputType::Color => "color",
        }
    };
    let align_class = move || match align.get() {
        InputAlign::Left => "",
        InputAlign::Center => "weui-input--center",
        InputAlign::Right => "weui-input--right",
    };
    let handle_input = move |ev: ev::Event| {
        let target = event_target::<web_sys::HtmlInputElement>(&ev);
        on_input.call(target.value());
    };
    let handle_clear = move |_: ev::MouseEvent| {
        on_input.call("".to_string());
        input_ref.get().map(|el| el.focus().ok());
    };
    let label_val = label.clone();
    let error_message_clone = error_message.clone();
    view! {
        <div
            class="weui-input-wrap"
            class=("weui-input-wrap--error", move || error.get())
            class=("weui-input-wrap--disabled", move || disabled.get())
        >
            <Show when=move || !label_val.get().is_empty()>
                <label class="weui-input__label">{label_val.get()}</label>
            </Show>
            <div class="weui-input__container">
                <input
                    node_ref=input_ref
                    type=type_attr
                    class=move || format!("weui-input {}", align_class())
                    class=("weui-input--error", move || error.get())
                    prop:value=value.clone()
                    placeholder=placeholder
                    disabled=disabled
                    readonly=readonly
                    required=required
                    style=style
                    on:input=handle_input
                />
                <Show when=move || show_clear()>
                    <button
                        class="weui-input__clear"
                        on:click=handle_clear
                        type="button"
                    >
                        <svg class="weui-icon weui-icon--xs" viewBox="0 0 24 24"><path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/></svg>
                    </button>
                </Show>
            </div>
            <Show when=move || error.get() && !error_message.get().is_empty()>
                <div class="weui-input__error">{error_message.get()}</div>
            </Show>
        </div>
    }
}

#[component]
pub fn TextArea(
    #[prop(into)] value: MaybeSignal<String>,
    #[prop(into)] on_input: Callback<String>,
    #[prop(into, default = "".into())] placeholder: MaybeSignal<String>,
    #[prop(into, default = false.into())] disabled: MaybeSignal<bool>,
    #[prop(into, default = false.into())] readonly: MaybeSignal<bool>,
    #[prop(into, default = false.into())] required: MaybeSignal<bool>,
    #[prop(into, default = false.into())] error: MaybeSignal<bool>,
    #[prop(into, default = false.into())] auto_height: MaybeSignal<bool>,
    #[prop(into, default = 0.into())] rows: MaybeSignal<i32>,
    #[prop(into, default = "".into())] label: MaybeSignal<String>,
    #[prop(into, default = "".into())] error_message: MaybeSignal<String>,
    #[prop(into, default = "".into())] class: MaybeSignal<String>,
    #[prop(into, default = "".into())] style: MaybeSignal<String>,
) -> impl IntoView {
    let handle_input = move |ev: ev::Event| {
        let target = event_target::<web_sys::HtmlTextAreaElement>(&ev);
        on_input.call(target.value());
    };
    let label_val = label.clone();
    let error_message_clone = error_message.clone();
    let value_clone = value.clone();
    view! {
        <div
            class="weui-textarea-wrap"
            class=("weui-textarea-wrap--error", move || error.get())
            class=("weui-textarea-wrap--disabled", move || disabled.get())
        >
            <Show when=move || !label_val.get().is_empty()>
                <label class="weui-textarea__label">{label_val.get()}</label>
            </Show>
            <textarea
                class=move || format!("weui-textarea {}", class.get())
                class=("weui-textarea--error", move || error.get())
                class=("weui-textarea--auto-height", move || auto_height.get())
                prop:value=value_clone
                placeholder=placeholder
                disabled=disabled
                readonly=readonly
                required=required
                rows=move || { if rows.get() > 0 { rows.get() } else { 3 } }
                style=style
                on:input=handle_input
            />
            <Show when=move || error.get() && !error_message_clone.get().is_empty()>
                <div class="weui-textarea__error">{error_message_clone.get()}</div>
            </Show>
        </div>
    }
}