use super::icon::{Icon, IconName};
use crate::theme::Size;
use leptos::*;
use gloo_timers::future::TimeoutFuture;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ToastType {
    Default,
    Success,
    Error,
    Warn,
    Loading,
}

#[derive(Debug, Clone)]
pub struct ToastConfig {
    pub id: String,
    pub message: String,
    pub toast_type: ToastType,
    pub duration: u32,
    pub position: ToastPosition,
    pub icon: Option<IconName>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ToastPosition {
    Top,
    Middle,
    Bottom,
}

impl ToastConfig {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            message: message.into(),
            toast_type: ToastType::Default,
            duration: 2000,
            position: ToastPosition::Middle,
            icon: None,
        }
    }

    pub fn success(mut self) -> Self {
        self.toast_type = ToastType::Success;
        self.icon = Some(IconName::Success);
        self
    }

    pub fn error(mut self) -> Self {
        self.toast_type = ToastType::Error;
        self.icon = Some(IconName::Close);
        self
    }

    pub fn warn(mut self) -> Self {
        self.toast_type = ToastType::Warn;
        self.icon = Some(IconName::Warn);
        self
    }

    pub fn loading(mut self) -> Self {
        self.toast_type = ToastType::Loading;
        self.icon = Some(IconName::Loading);
        self
    }

    pub fn duration(mut self, ms: u32) -> Self {
        self.duration = ms;
        self
    }

    pub fn position(mut self, pos: ToastPosition) -> Self {
        self.position = pos;
        self
    }

    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }
}

#[derive(Debug, Clone)]
pub enum ToastCommand {
    Show(ToastConfig),
    Hide(String),
    HideAll,
}

#[derive(Clone)]
pub struct ToastController {
    pub sender: RwSignal<Vec<ToastConfig>>,
}

impl ToastController {
    pub fn show(&self, config: ToastConfig) {
        self.sender.update(|toasts| {
            toasts.push(config.clone());
        });
        if config.duration > 0 {
            let id = config.id.clone();
            spawn_local(async move {
                TimeoutFuture::new(config.duration as u32).await;
            });
        }
    }

    pub fn hide(&self, id: &str) {
        self.sender.update(|toasts| {
            toasts.retain(|t| t.id != id);
        });
    }

    pub fn hide_all(&self) {
        self.sender.set(vec![]);
    }

    pub fn success(&self, message: impl Into<String>) {
        self.show(ToastConfig::new(message).success());
    }

    pub fn error(&self, message: impl Into<String>) {
        self.show(ToastConfig::new(message).error());
    }

    pub fn warn(&self, message: impl Into<String>) {
        self.show(ToastConfig::new(message).warn());
    }

    pub fn loading(&self, message: impl Into<String>) -> String {
        let config = ToastConfig::new(message).loading().duration(0);
        let id = config.id.clone();
        self.show(config);
        id
    }
}

pub fn provide_toast() {
    provide_context(ToastController {
        sender: RwSignal::new(vec![]),
    });
}

pub fn use_toast() -> ToastController {
    use_context::<ToastController>()
        .expect("ToastController not found")
}

#[component]
pub fn ToastContainer() -> impl IntoView {
    let toasts = use_toast();
    let items = toasts.sender;
    view! {
        <div class="weui-toast-container">
            <For
                each=move || items.get().clone()
                key=|t| t.id.clone()
                children=move |config: ToastConfig| {
                    let id = config.id.clone();
                    let toasts_clone = toasts.clone();
                    view! {
                        <ToastItem config=config on_close=move || toasts_clone.hide(&id)/>
                    }
                }
            />
        </div>
    }
}

#[component]
fn ToastItem(
    config: ToastConfig,
    on_close: impl Fn() + 'static,
) -> impl IntoView {
    let visible = create_rw_signal(true);
    let icon = move || config.icon.unwrap_or(IconName::Info);
    let type_class = move || match config.toast_type {
        ToastType::Default => "weui-toast--default",
        ToastType::Success => "weui-toast--success",
        ToastType::Error => "weui-toast--error",
        ToastType::Warn => "weui-toast--warn",
        ToastType::Loading => "weui-toast--loading",
    };
    let position_class = move || match config.position {
        ToastPosition::Top => "weui-toast--top",
        ToastPosition::Middle => "weui-toast--middle",
        ToastPosition::Bottom => "weui-toast--bottom",
    };
    if config.duration > 0 {
        let duration = config.duration;
        spawn_local(async move {
            TimeoutFuture::new(duration).await;
            visible.set(false);
            on_close();
        });
    }
    view! {
        <div
            class=move || format!("weui-toast {} {} {}", type_class(), position_class(), if visible.get() { "weui-toast--visible" } else { "weui-toast--hidden" })
            role="alert"
            aria-live="assertive"
            aria-atomic="true"
        >
            <div class="weui-toast__icon">
                <Icon name=icon size=Size::Lg/>
            </div>
            <div class="weui-toast__text">{config.message}</div>
        </div>
    }
}

#[component]
pub fn ToastProvider(children: Children) -> impl IntoView {
    provide_toast();
    view! {
        {children()}
        <ToastContainer/>
    }
}
