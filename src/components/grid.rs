use leptos::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GridDirection {
    Row,
    Column,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GridAlign {
    Start,
    Center,
    End,
    Stretch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GridJustify {
    Start,
    Center,
    End,
    SpaceBetween,
    SpaceAround,
}

#[component]
pub fn Grid(
    #[prop(into, default = 3.into())] columns: MaybeSignal<i32>,
    #[prop(into, default = "".into())] gap: MaybeSignal<String>,
    #[prop(into)] direction: MaybeSignal<GridDirection>,
    #[prop(into)] align: MaybeSignal<GridAlign>,
    #[prop(into)] justify: MaybeSignal<GridJustify>,
    #[prop(into, default = false.into())] square: MaybeSignal<bool>,
    #[prop(into, default = "".into())] class: MaybeSignal<String>,
    #[prop(into, default = "".into())] style: MaybeSignal<String>,
    children: Children,
) -> impl IntoView {
    let direction_class = move || match direction.get() {
        GridDirection::Row => "",
        GridDirection::Column => "weui-grid--column",
    };
    let align_class = move || match align.get() {
        GridAlign::Start => "weui-grid--align-start",
        GridAlign::Center => "weui-grid--align-center",
        GridAlign::End => "weui-grid--align-end",
        GridAlign::Stretch => "",
    };
    let justify_class = move || match justify.get() {
        GridJustify::Start => "weui-grid--justify-start",
        GridJustify::Center => "weui-grid--justify-center",
        GridJustify::End => "weui-grid--justify-end",
        GridJustify::SpaceBetween => "weui-grid--justify-between",
        GridJustify::SpaceAround => "weui-grid--justify-around",
    };
    let grid_style = move || {
        let cols = columns.get();
        let gap_val = gap.get();
        let gap_style = if gap_val.is_empty() {
            "".to_string()
        } else {
            format!("gap: {};", gap_val)
        };
        format!("grid-template-columns: repeat({}, 1fr); {}", cols, gap_style)
    };
    view! {
        <div
            class=move || format!("weui-grid {} {} {} {}", direction_class(), align_class(), justify_class(), class.get())
            class:weui-grid--square=move || square.get()
            style=move || format!("{} {}", grid_style(), style.get())
            role="grid"
        >
            {children()}
        </div>
    }
}

#[component]
pub fn GridItem(
    #[prop(into, default = 1.into())] span: MaybeSignal<i32>,
    #[prop(into, default = "".into())] class: MaybeSignal<String>,
    #[prop(into, default = "".into())] style: MaybeSignal<String>,
    children: Children,
) -> impl IntoView {
    let item_style = move || {
        let s = span.get();
        if s > 1 {
            format!("grid-column: span {};", s)
        } else {
            "".to_string()
        }
    };
    view! {
        <div
            class=move || format!("weui-grid__item {}", class.get())
            style=move || format!("{} {}", item_style(), style.get())
            role="gridcell"
        >
            {children()}
        </div>
    }
}
