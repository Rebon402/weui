use super::icon::{Icon, IconName};
use crate::theme::Size;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputDisplay {
    Normal,
    Floating,
    Stacked,
    StackedFloating,
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
    #[prop(into, default = "".into())] icon_left: MaybeSignal<String>,
    #[prop(into, default = "".into())] icon_right: MaybeSignal<String>,
    #[prop(into, default = "".into())] class: MaybeSignal<String>,
    #[prop(into, default = "".into())] style: MaybeSignal<String>,
    #[prop(into, default = 0.into())] max_length: MaybeSignal<i32>,
    #[prop(into, default = 0.into())] min_length: MaybeSignal<i32>,
    #[prop(into, default = "".into())] pattern: MaybeSignal<String>,
    #[prop(into, default = "".into())] autocomplete: MaybeSignal<String>,
    #[prop(into, default = "".into())] name: MaybeSignal<String>,
    #[prop(into, default = "".into())] inputmode: MaybeSignal<String>,
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
    let handle_focus = move |_: ev::FocusEvent| {};
    let handle_blur = move |_: ev::FocusEvent| {};
    view! {
        <div
            class="weui-input-wrap"
            class:weui-input-wrap--error=move || error.get()
            class:weui-input-wrap--disabled=move || disabled.get()
            class:weui-input-wrap--readonly=move || readonly.get()
        >
            {move || {
                if !label.get().is_empty() {
                    view! {
                        <label class="weui-input__label">{label.get()}</label>
                    }
                } else {
                    view! {}
                }
            }}
            <div class="weui-input__container">
                {move || {
                    if !icon_left.get().is_empty() {
                        view! { <span class="weui-input__icon-left"><Icon name=IconName::Search/></span> }
                    } else {
                        view! {}
                    }
                }}
                <input
                    node_ref=input_ref
                    type=type_attr
                    class=move || format!("weui-input {} {}", align_class(), class.get())
                    class:weui-input--error=move || error.get()
                    class:weui-input--disabled=move || disabled.get()
                    prop:value=value
                    placeholder=placeholder
                    disabled=disabled
                    readonly=readonly
                    required=required
                    maxlength=move || if max_length.get() > 0 { max_length.get() } else { -1 }
                    minlength=move || if min_length.get() > 0 { min_length.get() } else { -1 }
                    pattern=move || if pattern.get().is_empty() { None } else { Some(pattern.get()) }
                    autocomplete=move || if autocomplete.get().is_empty() { None } else { Some(autocomplete.get()) }
                    name=move || if name.get().is_empty() { None } else { Some(name.get()) }
                    inputmode=move || if inputmode.get().is_empty() { None } else { Some(inputmode.get()) }
                    style=style
                    on:input=handle_input
                    on:focus=handle_focus
                    on:blur=handle_blur
                />
                {move || {
                    if show_clear() {
                        view! {
                            <button
                                class="weui-input__clear"
                                on:click=handle_clear
                                type="button"
                                aria-label="Clear input"
                            >
                                <Icon name=IconName::Clear size=Size::Sm/>
                            </button>
                        }
                    } else {
                        view! {}
                    }
                }}
                {move || {
                    if !icon_right.get().is_empty() {
                        view! { <span class="weui-input__icon-right"><Icon name=IconName::Search/></span> }
                    } else {
                        view! {}
                    }
                }}
            </div>
            {move || {
                if error.get() && !error_message.get().is_empty() {
                    view! { <div class="weui-input__error">{error_message.get()}</div> }
                } else {
                    view! {}
                }
            }}
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
    #[prop(into, default = 0.into())] max_length: MaybeSignal<i32>,
    #[prop(into, default = "".into())] label: MaybeSignal<String>,
    #[prop(into, default = "".into())] error_message: MaybeSignal<String>,
    #[prop(into, default = "".into())] class: MaybeSignal<String>,
    #[prop(into, default = "".into())] style: MaybeSignal<String>,
) -> impl IntoView {
    let handle_input = move |ev: ev::Event| {
        let target = event_target::<web_sys::HtmlTextAreaElement>(&ev);
        on_input.call(target.value());
    };
    view! {
        <div
            class="weui-textarea-wrap"
            class:weui-textarea-wrap--error=move || error.get()
            class:weui-textarea-wrap--disabled=move || disabled.get()
        >
            {move || {
                if !label.get().is_empty() {
                    view! { <label class="weui-textarea__label">{label.get()}</label> }
                } else {
                    view! {}
                }
            }}
            <textarea
                class=move || format!("weui-textarea {}", class.get())
                class:weui-textarea--error=move || error.get()
                class:weui-textarea--auto-height=move || auto_height.get()
                prop:value=value
                placeholder=placeholder
                disabled=disabled
                readonly=readonly
                required=required
                rows=move || if rows.get() > 0 { rows.get() } else { 3 }
                maxlength=move || if max_length.get() > 0 { max_length.get() } else { -1 }
                style=style
                on:input=handle_input
            />
            {move || {
                if max_length.get() > 0 {
                    view! {
                        <div class="weui-textarea__counter">
                            {move || format!("{}/{}", value.get().len(), max_length.get())}
                        </div>
                    }
                } else {
                    view! {}
                }
            }}
            {move || {
                if error.get() && !error_message.get().is_empty() {
                    view! { <div class="weui-textarea__error">{error_message.get()}</div> }
                } else {
                    view! {}
                }
            }}
        </div>
    }
}
