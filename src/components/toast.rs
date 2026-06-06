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
    pub icon: Option<String>,
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
        self.icon = Some("success".into());
        self
    }

    pub fn error(mut self) -> Self {
        self.toast_type = ToastType::Error;
        self.icon = Some("close".into());
        self
    }

    pub fn warn(mut self) -> Self {
        self.toast_type = ToastType::Warn;
        self.icon = Some("warn".into());
        self
    }

    pub fn loading(mut self) -> Self {
        self.toast_type = ToastType::Loading;
        self.icon = Some("loading".into());
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

    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ToastPosition {
    Top,
    Middle,
    Bottom,
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
            let _id = config.id.clone();
            let duration = config.duration;
            spawn_local(async move {
                TimeoutFuture::new(duration).await;
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
                    let sender_clone = items;
                    view! {
                        <ToastView config=config on_close=move || sender_clone.update(|t| t.retain(|x| x.id != id))/>
                    }
                }
            />
        </div>
    }
}

#[component]
fn ToastView(
    config: ToastConfig,
    on_close: impl Fn(),
) -> impl IntoView {
    view! {
        <div
            class="weui-toast"
            class=("weui-toast--default", move || config.toast_type == ToastType::Default)
            class=("weui-toast--success", move || config.toast_type == ToastType::Success)
            class=("weui-toast--error", move || config.toast_type == ToastType::Error)
            class=("weui-toast--warn", move || config.toast_type == ToastType::Warn)
            class=("weui-toast--loading", move || config.toast_type == ToastType::Loading)
            role="alert"
            aria-live="assertive"
            aria-atomic="true"
        >
            <div class="weui-toast__icon">
                {config.icon.as_ref().map(|_| view! { <span class="weui-toast__svg-icon"/> })}
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