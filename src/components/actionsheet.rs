use leptos::*;

#[derive(Debug, Clone)]
pub struct ActionSheetAction {
    pub label: String,
    pub value: String,
    pub icon: Option<String>,
    pub disabled: bool,
    pub destructive: bool,
    pub callback: Option<String>,
}

impl ActionSheetAction {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            icon: None,
            disabled: false,
            destructive: false,
            callback: None,
        }
    }

    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn destructive(mut self) -> Self {
        self.destructive = true;
        self
    }
}

#[derive(Debug, Clone)]
pub struct ActionSheetConfig {
    pub title: String,
    pub actions: Vec<ActionSheetAction>,
    pub cancel_text: String,
    pub close_on_overlay: bool,
    pub close_on_select: bool,
}

impl ActionSheetConfig {
    pub fn new() -> Self {
        Self {
            title: "".into(),
            actions: vec![],
            cancel_text: "Cancel".into(),
            close_on_overlay: true,
            close_on_select: true,
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn action(mut self, action: ActionSheetAction) -> Self {
        self.actions.push(action);
        self
    }

    pub fn cancel_text(mut self, text: impl Into<String>) -> Self {
        self.cancel_text = text.into();
        self
    }

    pub fn close_on_overlay(mut self, close: bool) -> Self {
        self.close_on_overlay = close;
        self
    }

    pub fn close_on_select(mut self, close: bool) -> Self {
        self.close_on_select = close;
        self
    }
}

impl Default for ActionSheetConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
pub struct ActionSheetController {
    pub current: RwSignal<Option<ActionSheetConfig>>,
    pub visible: RwSignal<bool>,
}

#[component]
pub fn ActionSheetContainer() -> impl IntoView {
    let sheet = use_context::<ActionSheetController>()
        .expect("ActionSheetController not found");
    view! {
        <Show when=move || sheet.visible.get()>
            <ActionSheetView config=sheet.current.into() visible=sheet.visible/>
        </Show>
    }
}

#[component]
fn ActionSheetView(
    config: MaybeSignal<Option<ActionSheetConfig>>,
    visible: RwSignal<bool>,
) -> impl IntoView {
    let close_on_select_val = move || config.get().as_ref().map(|c| c.close_on_select).unwrap_or(true);
    let handle_overlay_click = move |ev: ev::MouseEvent| {
        ev.stop_propagation();
        let close_on_overlay = config.get().as_ref().map(|c| c.close_on_overlay).unwrap_or(true);
        if close_on_overlay {
            visible.set(false);
        }
    };
    let handle_keydown = move |ev: ev::KeyboardEvent| {
        if ev.key() == "Escape" {
            visible.set(false);
        }
    };
    view! {
        <div
            class="weui-actionsheet-overlay"
            class=("weui-actionsheet-overlay--visible", move || visible.get())
            on:click=handle_overlay_click
            on:keydown=handle_keydown
            role="dialog"
            aria-modal="true"
        >
            <div
                class="weui-actionsheet"
                role="menu"
            >
                {move || config.get().map(|cfg| view! {
                    <div class="weui-actionsheet__inner">
                        <Show when=move || !cfg.title.is_empty()>
                            <div class="weui-actionsheet__title">{cfg.title.clone()}</div>
                        </Show>
                        <div class="weui-actionsheet__menu">
                            {cfg.actions.iter().map(|action| {
                                let value = action.value.clone();
                                let close_on_sel = close_on_select_val();
                                view! {
                                    <button
                                        class="weui-actionsheet__item"
                                        class=("weui-actionsheet__item--destructive", move || action.destructive)
                                        class=("weui-actionsheet__item--disabled", move || action.disabled)
                                        disabled=move || action.disabled
                                        on:click=move |_| {
                                            if close_on_sel {
                                                visible.set(false);
                                            }
                                        }
                                        role="menuitem"
                                    >
                                        <span class="weui-actionsheet__item-label">{action.label.clone()}</span>
                                    </button>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                        <div class="weui-actionsheet__action">
                            <button
                                class="weui-actionsheet__cancel"
                                on:click=move |_| visible.set(false)
                            >
                                {cfg.cancel_text.clone()}
                            </button>
                        </div>
                    </div>
                })}
            </div>
        </div>
    }
}

pub fn provide_actionsheet() {
    provide_context(ActionSheetController {
        current: RwSignal::new(None),
        visible: RwSignal::new(false),
    });
}

pub fn use_actionsheet() -> ActionSheetController {
    use_context::<ActionSheetController>()
        .expect("ActionSheetController not found")
}

#[component]
pub fn ActionSheetProvider(children: Children) -> impl IntoView {
    provide_actionsheet();
    view! {
        {children()}
        <ActionSheetContainer/>
    }
}