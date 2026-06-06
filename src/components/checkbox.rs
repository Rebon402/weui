use leptos::*;
use leptos::Show;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CheckboxShape {
    Square,
    Round,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CheckboxSize {
    Small,
    Medium,
    Large,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CheckboxLabelPosition {
    Left,
    Right,
}

#[component]
pub fn Checkbox(
    #[prop(into)] checked: MaybeSignal<bool>,
    #[prop(into)] on_change: Callback<bool>,
    #[prop(into, default = false.into())] disabled: MaybeSignal<bool>,
    #[prop(into, default = false.into())] readonly: MaybeSignal<bool>,
    #[prop(into, default = false.into())] indeterminate: MaybeSignal<bool>,
    #[prop(into, default = "".into())] label: MaybeSignal<String>,
    #[prop(into, default = CheckboxShape::Square.into())] shape: MaybeSignal<CheckboxShape>,
    #[prop(into, default = CheckboxSize::Medium.into())] size: MaybeSignal<CheckboxSize>,
    #[prop(into, default = CheckboxLabelPosition::Right.into())] label_position: MaybeSignal<CheckboxLabelPosition>,
    #[prop(into, default = "".into())] class: MaybeSignal<String>,
    #[prop(into, default = "".into())] style: MaybeSignal<String>,
    #[prop(into, default = "".into())] name: MaybeSignal<String>,
    #[prop(into, default = "".into())] value: MaybeSignal<String>,
) -> impl IntoView {
    let input_ref = create_node_ref::<html::Input>();
    let label_val = label.clone();
    let label_val2 = label.clone();
    let check_ind = indeterminate;
    let check_val = checked;
    let shape_val = shape;
    let label_position_clone = label_position.clone();
    let shape_class = move || match shape.get() {
        CheckboxShape::Square => "weui-checkbox--square",
        CheckboxShape::Round => "weui-checkbox--round",
    };
    let size_class = move || match size.get() {
        CheckboxSize::Small => "weui-checkbox--small",
        CheckboxSize::Medium => "",
        CheckboxSize::Large => "weui-checkbox--large",
    };
    let handle_change = move |ev: ev::Event| {
        if disabled.get() || readonly.get() {
            ev.prevent_default();
            return;
        }
        let target = event_target::<web_sys::HtmlInputElement>(&ev);
        on_change.call(target.checked());
    };
    let handle_click = move |ev: ev::MouseEvent| {
        if label_position.get() == CheckboxLabelPosition::Left {
            ev.prevent_default();
            input_ref.get().map(|el| el.click());
        }
    };
    let handle_keydown = move |ev: ev::KeyboardEvent| {
        if ev.key() == " " || ev.key() == "Enter" {
            if !disabled.get() && !readonly.get() {
                ev.prevent_default();
                on_change.call(!checked.get());
            }
        }
    };
    let label_position_clone2 = label_position.clone();
    view! {
        <label
            class=move || format!("weui-checkbox {} {} {}", shape_class(), size_class(), class.get())
            class=("weui-checkbox--disabled", move || disabled.get())
            class=("weui-checkbox--checked", move || checked.get())
            class=("weui-checkbox--indeterminate", move || indeterminate.get())
            style=style
            on:click=handle_click
        >
            <Show when=move || label_position_clone.get() == CheckboxLabelPosition::Left>
                <span class="weui-checkbox__label weui-checkbox__label--left">{label_val.get()}</span>
            </Show>
            <input
                node_ref=input_ref
                type="checkbox"
                class="weui-checkbox__input"
                prop:checked=checked
                disabled=disabled
                readonly=readonly
                name=move || if name.get().is_empty() { None } else { Some(name.get()) }
                value=value
                on:change=handle_change
                on:keydown=handle_keydown
            />
            <span class="weui-checkbox__icon" aria-hidden="true">
                <Show when=move || check_ind.get()>
                    <svg class="weui-icon weui-icon--xs" viewBox="0 0 24 24"><path d="M19 13H5v-2h14v2z"/></svg>
                </Show>
                <Show when=move || !check_ind.get() && check_val.get()>
                    <svg class="weui-icon weui-icon--xs" viewBox="0 0 24 24"><path d="M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41z"/></svg>
                </Show>
                <Show when=move || !check_ind.get() && !check_val.get() && shape_val.get() == CheckboxShape::Round>
                    <svg class="weui-icon weui-icon--xs" viewBox="0 0 24 24"><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2z"/></svg>
                </Show>
            </span>
            <Show when=move || label_position_clone2.get() == CheckboxLabelPosition::Right>
                <span class="weui-checkbox__label">{label_val2.get()}</span>
            </Show>
        </label>
    }
}

#[component]
pub fn CheckboxGroup(
    #[prop(into)] checked_values: MaybeSignal<Vec<String>>,
    #[prop(into)] on_change: Callback<Vec<String>>,
    #[prop(into, default = false.into())] disabled: MaybeSignal<bool>,
    #[prop(into, default = 0usize.into())] max_selection: MaybeSignal<usize>,
    #[prop(into, default = CheckboxDirection::Vertical.into())] direction: MaybeSignal<CheckboxDirection>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            class="weui-checkbox-group"
            class=("weui-checkbox-group--vertical", move || direction.get() == CheckboxDirection::Vertical)
            class=("weui-checkbox-group--horizontal", move || direction.get() == CheckboxDirection::Horizontal)
            class=("weui-checkbox-group--disabled", move || disabled.get())
            role="group"
        >
            {children()}
        </div>
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CheckboxDirection {
    Vertical,
    Horizontal,
}

#[component]
pub fn Radio(
    #[prop(into)] checked: MaybeSignal<bool>,
    #[prop(into)] on_change: Callback<bool>,
    #[prop(into, default = false.into())] disabled: MaybeSignal<bool>,
    #[prop(into, default = "".into())] label: MaybeSignal<String>,
    #[prop(into, default = CheckboxSize::Medium.into())] size: MaybeSignal<CheckboxSize>,
    #[prop(into, default = "".into())] class: MaybeSignal<String>,
    #[prop(into, default = "".into())] style: MaybeSignal<String>,
    #[prop(into, default = "".into())] name: MaybeSignal<String>,
    #[prop(into, default = "".into())] value: MaybeSignal<String>,
) -> impl IntoView {
    let size_class = move || match size.get() {
        CheckboxSize::Small => "weui-radio--small",
        CheckboxSize::Medium => "",
        CheckboxSize::Large => "weui-radio--large",
    };
    let label_val = label.clone();
    let label_val2 = label.clone();
    let size_class_clone = size_class.clone();
    let class_clone = class.clone();
    view! {
        <label
            class=move || format!("weui-radio {} {}", size_class_clone(), class_clone.get())
            class=("weui-radio--disabled", move || disabled.get())
            class=("weui-radio--checked", move || checked.get())
            style=style
        >
            <input
                type="radio"
                class="weui-radio__input"
                prop:checked=checked
                disabled=disabled
                name=name
                value=value
                hidden=true
            />
            <span class="weui-radio__icon" aria-hidden="true"></span>
            <Show when=move || !label_val.get().is_empty()>
                <span class="weui-radio__label">{label_val2.get()}</span>
            </Show>
        </label>
    }
}
