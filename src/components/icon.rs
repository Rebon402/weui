use crate::theme::Size;
use leptos::*;

#[component]
pub fn Icon(
    #[prop(into)] name: MaybeSignal<IconName>,
    #[prop(into, default = Size::Md.into())] size: MaybeSignal<Size>,
    #[prop(into, default = "".into())] color: MaybeSignal<String>,
) -> impl IntoView {
    let path = move || name.get().svg_path();
    let sz = move || size.get().icon_size().to_string();
    let c = move || if color.get().is_empty() { "currentColor".to_string() } else { color.get() };
    view! {
        <svg
            class="weui-icon"
            width=sz.clone()
            height=sz
            viewBox="0 0 24 24"
            fill=c
            aria-hidden="true"
            focusable="false"
            role="img"
        >
            <path d=path/>
        </svg>
    }
}

#[component]
pub fn IconButton(
    #[prop(into)] name: MaybeSignal<IconName>,
    #[prop(into, default = false.into())] disabled: MaybeSignal<bool>,
) -> impl IntoView {
    view! {
        <button
            class="weui-icon-btn"
            disabled=move || disabled.get()
            type="button"
        >
            <Icon name=name size=Size::Md/>
        </button>
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IconName {
    Back,
    Arrow,
    Close,
    Search,
    Plus,
    Minus,
    Check,
    Circle,
    Info,
    Success,
    Waiting,
    Warn,
    Clear,
    Loading,
    More,
    Filter,
    FilterAlt,
}

impl IconName {
    pub fn svg_path(&self) -> &'static str {
        match self {
            Self::Back => "M20 11H7.83l5.59-5.59L12 4l-8 8 8 8 1.41-1.41L7.83 13H20v-2z",
            Self::Arrow => "M12 4l-1.41 1.41L16.17 11H4v2h12.17l-5.58 5.59L12 20l8-8z",
            Self::Close => "M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z",
            Self::Search => "M15.5 14h-.79l-.28-.27C15.41 12.59 16 11.11 16 9.5 16 5.91 13.09 3 9.5 3S3 5.91 3 9.5 5.91 16 9.5 16c1.61 0 3.09-.59 4.23-1.57l.27.28v.79l5 4.99L20.49 19l-4.99-5zm-6 0C7.01 14 5 11.99 5 9.5S7.01 5 9.5 5 14 7.01 14 9.5 11.99 14 9.5 14z",
            Self::Plus => "M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z",
            Self::Minus => "M19 13H5v-2h14v2z",
            Self::Check => "M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41z",
            Self::Circle => "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2z",
            Self::Info => "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 15h-2v-6h2v6zm0-8h-2V7h2v2z",
            Self::Success => "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z",
            Self::Waiting => "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm.5 4v4.25l3.5 2.08-.76 1.24L11 11V6h1.5z",
            Self::Warn => "M1 21h22L12 2 1 21zm12-3h-2v-2h2v2zm0-4h-2v-4h2v4z",
            Self::Clear => "M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z",
            Self::Loading => "M12 4V2.21c0-.45-.54-.67-.85-.35l-2.8 2.8c-.2.2-.2.51 0 .71l2.8 2.8c.31.31.85.09.85-.35V6c3.31 0 6 2.69 6 6 0 1.01-.25 1.97-.7 2.8l1.06 1.06C20.1 14.6 21 12.86 21 11c0-4.42-3.58-8-8-8zm0 14c-3.31 0-6-2.69-6-6 0-1.01.25-1.97.7-2.8L5.64 8.14C4.4 9.4 3.5 11.14 3.5 13c0 4.42 3.58 8 8 8v1.79c0 .45.54.67.85.35l2.8-2.8c.2-.2.2-.51 0-.71l-2.8-2.8c-.31-.31-.85-.09-.85.35V18z",
            Self::More => "M6 10c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm12 0c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm-6 0c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2z",
            Self::Filter => "M10 18h4v-2h-4v2zM3 6v2h18V6H3zm3 7h12v-2H6v2z",
            Self::FilterAlt => "M4.25 5.61C6.27 8.2 10 13 10 13v6c0 .55.45 1 1 1h2c.55 0 1-.45 1-1v-6s3.72-4.8 5.74-7.39c.51-.67.04-1.61-.74-1.61H5.02c-.78 0-1.25.95-.77 1.61z",
        }
    }
}
