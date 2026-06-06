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
    #[prop(into, default = None.into())] on_search: Option<Callback<String>>,
    #[prop(into, default = None.into())] on_cancel: Option<Callback<()>>,
    #[prop(into, default = None.into())] on_clear: Option<Callback<()>>,
) -> impl IntoView {
    let input_ref = create_node_ref::<html::Input>();
    let has_value = create_memo(move |_| !value.get().is_empty());
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
    let handle_keydown = move |ev: ev::KeyboardEvent| {
        if ev.key() == "Enter" {
            if let Some(cb) = &on_search {
                cb.call(value.get());
            }
        }
    };
    let handle_clear = move |_: ev::MouseEvent| {
        on_input.call("".to_string());
        if let Some(cb) = &on_clear {
            cb.call(());
        }
        input_ref.get().map(|el| el.focus().ok());
    };
    let handle_cancel = move |_: ev::MouseEvent| {
        on_input.call("".to_string());
        if let Some(cb) = &on_cancel {
            cb.call(());
        }
    };
    view! {
        <div
            class=move || format!("weui-search {} {}", shape_class(), class.get())
            class:weui-search--focus=move || focus.get()
            class:weui-search--disabled=move || disabled.get()
        >
            <div class="weui-search__icon"/>
            <input
                node_ref=input_ref
                type="search"
                class="weui-search__input"
                prop:value=value
                placeholder=placeholder
                disabled=disabled
                on:input=handle_input
                on:keydown=handle_keydown
                role="searchbox"
            />
            {move || {
                if show_clear() {
                    view! {
                        <button
                            class="weui-search__clear"
                            on:click=handle_clear
                            type="button"
                            aria-label="Clear search"
                        />
                    }
                } else {
                    view! {}
                }
            }}
            {move || {
                if let Some(_) = &on_cancel {
                    view! {
                        <button
                            class="weui-search__cancel"
                            on:click=handle_cancel
                            type="button"
                        >
                            "Cancel"
                        </button>
                    }
                } else {
                    view! {}
                }
            }}
        </div>
    }
}
