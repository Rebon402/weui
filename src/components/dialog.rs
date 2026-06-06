use leptos::*;
use leptos::Show;

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

#[derive(Debug, Clone, PartialEq)]
pub struct DialogAction {
    pub label: String,
    pub variant: DialogActionVariant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DialogActionVariant {
    Default,
    Primary,
    Warn,
}

impl DialogAction {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            variant: DialogActionVariant::Default,
        }
    }

    pub fn primary(mut self) -> Self {
        self.variant = DialogActionVariant::Primary;
        self
    }

    pub fn warn(mut self) -> Self {
        self.variant = DialogActionVariant::Warn;
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

#[derive(Clone)]
pub struct DialogController {
    pub current: RwSignal<Option<DialogConfig>>,
    pub visible: RwSignal<bool>,
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
pub fn DialogContainer() -> impl IntoView {
    let dialog = use_dialog();
    view! {
        <Show when=move || dialog.visible.get()>
            <Show when=move || dialog.current.get().is_some()>
                <div
                    class="weui-dialog-overlay"
                    class=("weui-dialog-overlay--visible", move || dialog.visible.get())
                    on:click=move |ev: ev::MouseEvent| {
                        ev.stop_propagation();
                        if dialog.current.get().as_ref().map(|c| c.close_on_overlay).unwrap_or(true) {
                            dialog.visible.set(false);
                        }
                    }
                    role="dialog"
                    aria-modal="true"
                >
                    <div
                        class="weui-dialog"
                        class=("weui-dialog--large", move || matches!(dialog.current.get().as_ref().map(|c| c.size), Some(DialogSize::Large)))
                        class=("weui-dialog--small", move || matches!(dialog.current.get().as_ref().map(|c| c.size), Some(DialogSize::Small)))
                    >
                        {move || dialog.current.get().map(|cfg| {
                            let cfg = cfg.clone();
                            let title = cfg.title.clone();
                            let has_title = !title.is_empty();
                            let message = cfg.message.clone();
                            view! {
                                <div class="weui-dialog__body">
                                    <Show when=move || has_title>
                                        <div class="weui-dialog__title" id="dialog-title">{title.clone()}</div>
                                    </Show>
                                    <div class="weui-dialog__content" id="dialog-content">{message.clone()}</div>
                                </div>
                                <div class="weui-dialog__actions">
                                    {cfg.actions.iter().map(|action| {
                                        let action = action.clone();
                                        view! {
                                            <button
                                                class="weui-dialog__action"
                                                on:click=move |_| dialog.visible.set(false)
                                            >
                                                {action.label.clone()}
                                            </button>
                                        }
                                    }).collect::<Vec<_>>()}
                                </div>
                            }
                        })}
                    </div>
                </div>
            </Show>
        </Show>
    }
}

#[component]
pub fn DialogProvider(children: Children) -> impl IntoView {
    provide_dialog();
    view! {
        {children()}
        <DialogContainer/>
    }
}