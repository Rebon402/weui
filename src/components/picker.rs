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

#[derive(Clone)]
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
    #[prop(into, default = "".into())] class: MaybeSignal<String>,
) -> impl IntoView {
    let class_clone = class.clone();
    let title_clone = title.clone();
    view! {
        <Show when=move || visible.get()>
            <div class=move || format!("weui-picker {}", class_clone.get())>
                <div class="weui-picker__header">
                    <button class="weui-picker__btn" type="button">
                        "Cancel"
                    </button>
                    <span class="weui-picker__title">{title_clone.get()}</span>
                    <button class="weui-picker__btn weui-picker__btn--primary" type="button">
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
                                    <div class="weui-picker__column">
                                        {col.values.iter().enumerate().map(|(idx, value)| {
                                            view! { <div class="weui-picker__item">{value.clone()}</div> }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                }
                            })
                            .collect::<Vec<_>>()
                    }}
                </div>
            </div>
        </Show>
    }
}

pub type PickerColumnView = ();