use super::button::{Button, ButtonVariant};
use super::icon::{Icon, IconName};
use leptos::*;

#[derive(Debug, Clone)]
pub struct ActionSheetAction {
    pub label: String,
    pub value: String,
    pub icon: Option<IconName>,
    pub disabled: bool,
    pub destructive: bool,
    pub callback: Option<Callback<String>>,
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

    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
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

    pub fn on_select<F: Fn(String) + 'static>(mut self, f: F) -> Self {
        self.callback = Some(Callback::new(f));
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

#[derive(Debug, Clone)]
pub enum ActionSheetCommand {
    Show(ActionSheetConfig),
    Hide,
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
    let sheet_ref = create_node_ref::<html::Div>();
    let show_anim = create_rw_signal(false);
    create_effect(move |_| {
        sheet_ref.get().map(|_| show_anim.set(true));
    });
    let handle_overlay_click = move |ev: ev::MouseEvent| {
        ev.stop_propagation();
        if config.get().as_ref().map(|c| c.close_on_overlay).unwrap_or(true) {
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
                class=("weui-actionsheet--visible", move || show_anim.get())
                node_ref=sheet_ref
                role="menu"
            >
                {move || config.get().map(|cfg| view! {
                    <div class="weui-actionsheet__inner">
                        <Show when=move || !cfg.title.is_empty()>
                            <div class="weui-actionsheet__title">{cfg.title.clone()}</div>
                        </Show>
                        <div class="weui-actionsheet__menu">
                            {cfg.actions.into_iter().map(|action| {
                                let value = action.value.clone();
                                let label = action.label.clone();
                                let icon = action.icon;
                                let disabled = action.disabled;
                                let destructive = action.destructive;
                                let action_clone_for_cb = action.clone();
                                let visible_clone = visible;
                                let close_on_select = cfg.close_on_select;
                                view! {
                                    <button
                                        class="weui-actionsheet__item"
                                        class=("weui-actionsheet__item--destructive", destructive)
                                        class=("weui-actionsheet__item--disabled", disabled)
                                        disabled=disabled
                                        on:click=move |_| {
                                            if close_on_select {
                                                visible_clone.set(false);
                                            }
                                            if let Some(cb) = &action_clone_for_cb.callback {
                                                cb.call(value.clone());
                                            }
                                        }
                                        role="menuitem"
                                    >
                                        {icon.map(|i| view! { <span class="weui-actionsheet__item-icon"><Icon name=i/></span> }).collect::<Vec<_>>()}
                                        <span class="weui-actionsheet__item-label">{label}</span>
                                    </button>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                        <div class="weui-actionsheet__action">
                            <Button
                                variant=ButtonVariant::Default
                                block=true
                                on_click=move |_| visible.set(false)
                            >
                                {cfg.cancel_text.clone()}
                            </Button>
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
