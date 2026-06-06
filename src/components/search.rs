use leptos::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SearchShape {
    Square,
    Round,
}

#[component]
pub fn SearchBar(
    #[prop(into)] value: MaybeSignal<String>,
    #[prop(into)] on_input: Callback<String>,
    #[prop(into, default = "".into())] placeholder: MaybeSignal<String>,
    #[prop(into)] shape: MaybeSignal<SearchShape>,
    #[prop(into, default = true.into())] clearable: MaybeSignal<bool>,
    #[prop(into, default = false.into())] disabled: MaybeSignal<bool>,
    #[prop(into, default = false.into())] focus: MaybeSignal<bool>,
    #[prop(into, default = "".into())] class: MaybeSignal<String>,
) -> impl IntoView {
    let input_ref = create_node_ref::<html::Input>();
    let value_clone = value.clone();
    let has_value = create_memo(move |_| !value_clone.get().is_empty());
    let show_clear = create_memo(move |_| clearable.get() && has_value.get() && !disabled.get());
    create_effect(move |_| {
        if focus.get() {
            input_ref.get().map(|el| el.focus().ok());
        }
    });
    let shape_class = move || match shape.get() {
        SearchShape::Square => "",
        SearchShape::Round => "weui-search--round",
    };
    let handle_input = move |ev: ev::Event| {
        let target = event_target::<web_sys::HtmlInputElement>(&ev);
        on_input.call(target.value());
    };
    let handle_clear = move |_: ev::MouseEvent| {
        on_input.call("".to_string());
        input_ref.get().map(|el| el.focus().ok());
    };
    view! {
        <div
            class=move || format!("weui-search {} {}", shape_class(), class.get())
            class=("weui-search--focus", move || focus.get())
            class=("weui-search--disabled", move || disabled.get())
        >
            <div class="weui-search__icon"/>
            <input
                node_ref=input_ref
                type="search"
                class="weui-search__input"
                prop:value=value_clone
                placeholder=placeholder
                disabled=disabled
                on:input=handle_input
                role="searchbox"
            />
            <Show when=move || show_clear()>
                <button
                    class="weui-search__clear"
                    on:click=handle_clear
                    type="button"
                />
            </Show>
        </div>
    }
}