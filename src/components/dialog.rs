use super::button::{ButtonVariant, ButtonSize};
use super::icon::{Icon, IconName};
use crate::theme::Size;
use leptos::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DialogSize {
    Default,
    Large,
    Small,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DialogType {
    Alert,
    Confirm,
    Prompt,
}

#[derive(Debug, Clone)]
pub struct DialogAction {
    pub label: String,
    pub variant: ButtonVariant,
    pub callback: Option<Callback<(), ()>>,
}

impl DialogAction {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            variant: ButtonVariant::Default,
            callback: None,
        }
    }

    pub fn primary(mut self) -> Self {
        self.variant = ButtonVariant::Primary;
        self
    }

    pub fn warn(mut self) -> Self {
        self.variant = ButtonVariant::Warn;
        self
    }

    pub fn on_action<F: Fn() + 'static>(mut self, f: F) -> Self {
        self.callback = Some(Callback::new(move |()| f()));
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DialogConfig {
    pub title: String,
    pub message: String,
    pub actions: Vec<DialogAction>,
    pub size: DialogSize,
    pub close_on_overlay: bool,
    pub show_close: bool,
}

impl DialogConfig {
    pub fn new(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            message: message.into(),
            actions: vec![],
            size: DialogSize::Default,
            close_on_overlay: true,
            show_close: false,
        }
    }

    pub fn alert(message: impl Into<String>) -> Self {
        Self {
            title: "".into(),
            message: message.into(),
            actions: vec![DialogAction::new("OK").primary()],
            size: DialogSize::Default,
            close_on_overlay: true,
            show_close: false,
        }
    }

    pub fn confirm(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            message: message.into(),
            actions: vec![
                DialogAction::new("Cancel"),
                DialogAction::new("Confirm").primary(),
            ],
            size: DialogSize::Default,
            close_on_overlay: true,
            show_close: false,
        }
    }

    pub fn size(mut self, size: DialogSize) -> Self {
        self.size = size;
        self
    }

    pub fn close_on_overlay(mut self, close: bool) -> Self {
        self.close_on_overlay = close;
        self
    }

    pub fn show_close(mut self, show: bool) -> Self {
        self.show_close = show;
        self
    }

    pub fn action(mut self, action: DialogAction) -> Self {
        self.actions.push(action);
        self
    }
}

#[derive(Debug, Clone)]
pub enum DialogCommand {
    Show(DialogConfig),
    Hide,
}

#[derive(Clone)]
pub struct DialogController {
    pub current: RwSignal<Option<DialogConfig>>,
    pub visible: RwSignal<bool>,
}

#[component]
pub fn DialogContainer() -> impl IntoView {
    let dialog = use_context::<DialogController>()
        .expect("DialogController not found");
    view! {
        <Show when=move || dialog.visible.get()>
            <DialogView config=dialog.current.into() visible=dialog.visible/>
        </Show>
    }
}

#[component]
fn DialogView(
    config: MaybeSignal<Option<DialogConfig>>,
    visible: RwSignal<bool>,
) -> impl IntoView {
    let dialog_ref = create_node_ref::<html::Div>();
    create_effect(move |_| {
        if let Some(el) = dialog_ref.get() {
            let _ = el.focus();
        }
    });
    let handle_overlay_click = move |ev: ev::MouseEvent| {
        ev.stop_propagation();
        if let Some(ref cfg) = config.get() {
            if cfg.close_on_overlay {
                visible.set(false);
            }
        }
    };
    let handle_keydown = move |ev: ev::KeyboardEvent| {
        if ev.key() == "Escape" {
            visible.set(false);
        }
    };
    view! {
        <div
            class="weui-dialog-overlay"
            class=("weui-dialog-overlay--visible", move || visible.get())
            on:click=handle_overlay_click
            on:keydown=handle_keydown
            tabindex="-1"
            role="dialog"
            aria-modal="true"
            aria-labelledby="dialog-title"
            aria-describedby="dialog-content"
        >
            <div
                class="weui-dialog"
                class=("weui-dialog--large", move || matches!(config.get().as_ref().map(|c| c.size), Some(DialogSize::Large)))
                class=("weui-dialog--small", move || matches!(config.get().as_ref().map(|c| c.size), Some(DialogSize::Small)))
                node_ref=dialog_ref
            >
                {move || config.get().map(|cfg| view! {
                    <div class="weui-dialog__body">
                        <Show when=move || !cfg.title.is_empty()>
                            <div class="weui-dialog__title" id="dialog-title">{cfg.title.clone()}</div>
                        </Show>
                        <Show when=move || cfg.show_close>
                            <button
                                class="weui-dialog__close"
                                on:click=move |_| visible.set(false)
                                aria-label="Close dialog"
                                type="button"
                            >
                                <Icon name=IconName::Close size=Size::Lg/>
                            </button>
                        </Show>
                        <div class="weui-dialog__content" id="dialog-content">{cfg.message.clone()}</div>
                    </div>
                    <div class="weui-dialog__actions">
                        {cfg.actions.into_iter().map(|action| {
                            let label = action.label.clone();
                            let action_clone = action.clone();
                            let visible_clone = visible;
                            view! {
                                <Button
                                    variant=action.variant
                                    size=ButtonSize::Normal
                                    on_click=move |_| {
                                        visible_clone.set(false);
                                        if let Some(cb) = &action_clone.callback {
                                            cb.call(());
                                        }
                                    }
                                >
                                    {label}
                                </Button>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                })}
            </div>
        </div>
    }
}

pub fn provide_dialog() {
    provide_context(DialogController {
        current: RwSignal::new(None),
        visible: RwSignal::new(false),
    });
}

pub fn use_dialog() -> DialogController {
    use_context::<DialogController>()
        .expect("DialogController not found")
}

#[component]
pub fn DialogProvider(children: Children) -> impl IntoView {
    provide_dialog();
    view! {
        {children()}
        <DialogContainer/>
    }
}
