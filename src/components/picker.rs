use leptos::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PickerMode {
    Selector,
    MultiSelector,
    Date,
    Time,
    DateTime,
    Region,
}

#[derive(Debug, Clone)]
pub struct PickerColumn {
    pub values: Vec<String>,
    pub default_index: usize,
}

#[component]
pub fn Picker(
    #[prop(into)] visible: MaybeSignal<bool>,
    #[prop(into)] columns: MaybeSignal<Vec<PickerColumn>>,
    #[prop(into)] mode: MaybeSignal<PickerMode>,
    #[prop(into, default = "".into())] title: MaybeSignal<String>,
    #[prop(into, default = false.into())] loading: MaybeSignal<bool>,
    #[prop(into, default = "".into())] class: MaybeSignal<String>,
    #[prop(into, default = None.into())] on_confirm: Option<Callback<Vec<usize>>>,
    #[prop(into, default = None.into())] on_cancel: Option<Callback<()>>,
    #[prop(into, default = None.into())) on_change: Option<Callback<(usize, usize)>>,
) -> impl IntoView {
    let selected_indices = create_memo(move |_| {
        columns
            .get()
            .iter()
            .map(|col| col.default_index)
            .collect::<Vec<_>>()
    });
    let handle_confirm = move |_: ev::MouseEvent| {
        if let Some(cb) = &on_confirm {
            cb.call(selected_indices.get());
        }
    };
    let handle_cancel = move |_: ev::MouseEvent| {
        if let Some(cb) = &on_cancel {
            cb.call(());
        }
    };
    view! {
        <Show when=move || visible.get()>
            <div class=move || format!("weui-picker {}", class.get())>
                <div class="weui-picker__header">
                    <button
                        class="weui-picker__btn"
                        on:click=handle_cancel
                        type="button"
                    >
                        "Cancel"
                    </button>
                    <span class="weui-picker__title">{title}</span>
                    <button
                        class="weui-picker__btn weui-picker__btn--primary"
                        on:click=handle_confirm
                        type="button"
                    >
                        "Confirm"
                    </button>
                </div>
                <div class="weui-picker__body">
                    {move || {
                        columns
                            .get()
                            .into_iter()
                            .enumerate()
                            .map(|(col_idx, col)| {
                                view! {
                                    <PickerColumnView
                                        column=col
                                        index=col_idx
                                        on_change=on_change.clone()
                                    />
                                }
                            })
                            .collect::<Vec<_>>()
                    }}
                </div>
            </div>
        </Show>
    }
}

#[component]
fn PickerColumnView(
    column: PickerColumn,
    index: usize,
    #[prop(into, default = None.into())] on_change: Option<Callback<(usize, usize)>>,
) -> impl IntoView {
    let current_index = create_rw_signal(column.default_index);
    let handle_scroll = move |ev: ev::Event| {
        if let Some(cb) = &on_change {
            cb.call((index, current_index.get()));
        }
    };
    view! {
        <div
            class="weui-picker__column"
            on:scroll=handle_scroll
        >
            {column
                .values
                .into_iter()
                .enumerate()
                .map(|(idx, value)| {
                    let is_selected = move || current_index.get() == idx;
                    view! {
                        <div
                            class="weui-picker__item"
                            class:weui-picker__item--selected=is_selected
                        >
                            {value}
                        </div>
                    }
                })
                .collect::<Vec<_>>()}
        </div>
    }
}
